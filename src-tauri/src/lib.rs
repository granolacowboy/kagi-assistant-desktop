use tauri::{Manager, Window, Emitter, Listener};
use std::sync::{Arc, Mutex};

#[tauri::command]
fn send_url(url: &str) -> String {
    format!("URL received: {}", url)
}

#[tauri::command]
fn set_current_url(url: &str) -> String {
    format!("Current URL set to: {}", url)
}

#[tauri::command]
async fn go_to_assistant(window: Window) {
    println!("Manual redirection to assistant requested");
    if let Err(e) = window.emit("go-to-assistant", ()) {
        eprintln!("Failed to send go-to-assistant event: {}", e);
    }
}

#[tauri::command]
async fn show_window(window: Window) {
    if let Err(e) = window.show() {
        eprintln!("Failed to show window: {}", e);
    }
}

const REDIRECT_SCRIPT: &str = r#"
if (window.location.href.includes('/assistant')) {
    console.log('Already on Kagi Assistant page - no script injection needed');
} else if (!window.__KAGI_REDIRECTOR__) {
    console.log('🔄 KAGI REDIRECTION SCRIPT INJECTED');
    
    window.__KAGI_REDIRECTOR__ = {
        hasRedirected: false,
        debugMode: true,
        log: function(msg) {
            if (this.debugMode) console.log('🔄 KAGI: ' + msg);
        }
    };
    
    window.__KAGI_REDIRECTOR__.logElement = function(el) {
        if (!this.debugMode) return;
        try {
            console.log(el);
        } catch (e) {
            console.log('Could not log element: ' + e);
        }
    };
    
    window.__KAGI_REDIRECTOR__.redirectToAssistant = function() {
        if (this.hasRedirected) return;
        
        this.log('🚀 REDIRECTING TO ASSISTANT');
        this.hasRedirected = true;
        
        try {
            localStorage.setItem('kagi_logged_in', 'true');
            this.log('Saved login state to localStorage');
        } catch (e) {
            this.log('Error saving localStorage: ' + e);
        }
        
        try {
            window.location.href = 'https://kagi.com/assistant';
        } catch (e) {
            this.log('Main redirect failed: ' + e);
        }
    };
    
    window.__KAGI_REDIRECTOR__.isAssistantPage = function() {
        return window.location.href.includes('/assistant');
    };
    
    window.__KAGI_REDIRECTOR__.isHomePage = function() {
        const url = window.location.href;
        
        if (this.isAssistantPage()) {
            this.log('Already on assistant page');
            return false;
        }
        
        if (url === 'https://kagi.com' || 
            url === 'https://kagi.com/' || 
            url.startsWith('https://kagi.com/?') || 
            url.includes('kagi.com/search') ||
            url.includes('kagi.com/home')) {
            
            this.log('Homepage detected via URL: ' + url);
            try {
                localStorage.setItem('kagi_logged_in', 'true');
                this.log('Successfully saved login state to localStorage');
            } catch (e) {
                this.log('Failed to save login state: ' + e);
            }
            return true;
        }
        
        if (document.body) {
            try {
                const searchBar = document.querySelector('input[type="search"]') || 
                                 document.querySelector('.search-field') ||
                                 document.querySelector('input[name="q"]') ||
                                 document.getElementById('search-input');
                
                if (searchBar) {
                    this.log('Homepage detected via search bar');
                    try {
                        localStorage.setItem('kagi_logged_in', 'true');
                        this.log('Successfully saved login state to localStorage');
                    } catch (e) {
                        this.log('Failed to save login state: ' + e);
                    }
                    return true;
                }
                
                const navElements = document.querySelector('nav') || 
                                  document.querySelector('.navbar') ||
                                  document.querySelector('.header-nav');
                                  
                if (navElements && 
                    !url.includes('/signin') && 
                    !url.includes('/login') && 
                    !url.includes('/register')) {
                    this.log('Homepage detected via nav elements');
                    try {
                        localStorage.setItem('kagi_logged_in', 'true');
                        this.log('Successfully saved login state to localStorage');
                    } catch (e) {
                        this.log('Failed to save login state: ' + e);
                    }
                    return true;
                }
            } catch (e) {
                this.log('Error in DOM detection: ' + e);
            }
        }
        
        return false;
    };
    
    window.__KAGI_REDIRECTOR__.checkForRedirect = function() {
        if (this.hasRedirected) return;
        this.log('Checking for redirect conditions...');
        
        if (this.isHomePage()) {
            this.log('Homepage detected, redirecting...');
            
            try {
                localStorage.setItem('kagi_logged_in', 'true');
                this.log('Saved login state for future sessions');
            } catch (e) {
                this.log('Error saving login state: ' + e);
            }
            
            this.redirectToAssistant();
        } else {
            this.log('Not on homepage, URL: ' + window.location.href);
        }
    };
    
    window.__KAGI_REDIRECTOR__.createRedirectButton = function() {
        const self = this;
        
        if (!document.body) {
            this.log('Document body not ready, will retry adding button');
            setTimeout(function() { self.createRedirectButton(); }, 100);
            return;
        }
        
        if (document.querySelector('.kagi-redirect-btn')) {
            this.log('Button already exists');
            return;
        }
        
        try {
            const btnStyle = document.createElement('style');
            btnStyle.textContent = `
                .kagi-redirect-btn {
                    position: fixed;
                    bottom: 20px;
                    right: 20px;
                    background-color: #e12c2c;
                    color: white;
                    font-weight: bold;
                    padding: 15px 30px;
                    border-radius: 8px;
                    z-index: 999999;
                    cursor: pointer;
                    border: none;
                    font-size: 16px;
                    box-shadow: 0 4px 10px rgba(0,0,0,0.3);
                    font-family: system-ui, sans-serif;
                }
                .kagi-redirect-btn:hover {
                    background-color: #ff3c3c;
                    transform: translateY(-2px);
                    box-shadow: 0 6px 15px rgba(0,0,0,0.3);
                }
            `;
            document.head.appendChild(btnStyle);
            
            const btn = document.createElement('button');
            btn.className = 'kagi-redirect-btn';
            btn.textContent = 'GO TO ASSISTANT NOW';
            const redirector = this;
            btn.onclick = function() { redirector.redirectToAssistant(); };
            document.body.appendChild(btn);
            
            this.log('Redirect button created');
        } catch (e) {
            this.log('Error creating button: ' + e);
            setTimeout(function() { self.createRedirectButton(); }, 500);
        }
    };
    
    window.__KAGI_REDIRECTOR__.init = function() {
        const self = this;
        this.log('Initializing redirector...');
        
        try {
            const previouslyLoggedIn = localStorage.getItem('kagi_logged_in') === 'true';
            if (previouslyLoggedIn && !this.isAssistantPage()) {
                this.log('Previous login detected in localStorage, redirecting to assistant directly');
                this.redirectToAssistant();
                return;
            }
        } catch (e) {
            this.log('Error checking localStorage: ' + e);
        }
        
        this.checkForRedirect();
        
        if (document.readyState === 'loading') {
            this.log('Document still loading, waiting for DOMContentLoaded');
            document.addEventListener('DOMContentLoaded', function() {
                self.createRedirectButton();
            });
        } else {
            this.log('Document ready, creating button');
            this.createRedirectButton();
        }
        
        setInterval(function() { 
            self.checkForRedirect(); 
        }, 1000);
        
        setTimeout(function() { self.checkForRedirect(); }, 2000);
        setTimeout(function() { self.checkForRedirect(); }, 5000);
    };
}

if (window.location.href.includes('/assistant')) {
    console.log('On assistant page, no initialization needed');
} else if (window.__KAGI_REDIRECTOR__) {
    window.__KAGI_REDIRECTOR__.log('Redirector already exists, reinitializing');
    window.__KAGI_REDIRECTOR__.init();
} else {
    console.log('🔄 Creating new redirector');
    window.__KAGI_REDIRECTOR__ = { /* ... object initialization ... */ };
    window.__KAGI_REDIRECTOR__.init();
}
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            
            let quit = tauri::menu::MenuItemBuilder::with_id("quit", "Quit")
                .accelerator("CmdOrCtrl+Q")
                .build(app).unwrap();
            let show = tauri::menu::MenuItemBuilder::with_id("show", "Show")
                .build(app).unwrap();
            let hide = tauri::menu::MenuItemBuilder::with_id("hide", "Hide")
                .build(app).unwrap();
            
            let menu = tauri::menu::MenuBuilder::new(app)
                .items(&[&show, &hide, &quit])
                .build().unwrap();
            
            let _tray = tauri::tray::TrayIconBuilder::new()
                .tooltip("Kagi Assistant")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id().0.as_str() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.hide().unwrap();
                        }
                    }
                    _ => {}
                })
                .build(app).unwrap();
            
            #[cfg(debug_assertions)]
            {
                main_window.open_devtools();
                println!("Developer tools enabled");
            }
            
            std::thread::sleep(std::time::Duration::from_millis(500));
            
            println!("Injecting initial redirect script");
            if let Err(e) = main_window.eval(REDIRECT_SCRIPT) {
                eprintln!("Failed to inject initial script: {}", e);
            }
            
            let is_loading = Arc::new(Mutex::new(false));
            let on_assistant_page = Arc::new(Mutex::new(false));
            
            let window_clone = main_window.clone();
            let loading_flag = Arc::clone(&is_loading);
            let assistant_flag = Arc::clone(&on_assistant_page);
            
            main_window.listen("tauri://location-changed", move |event| {
                let url = event.payload();
                println!("Location changed to: {}", url);
                
                let on_assistant = url.contains("/assistant");
                {
                    let mut assistant = assistant_flag.lock().unwrap();
                    *assistant = on_assistant;
                    if on_assistant {
                        println!("Successfully navigated to assistant page - no further injections needed");
                    }
                }
                
                let mut loading = loading_flag.lock().unwrap();
                *loading = true;
                
                let window = window_clone.clone();
                let assistant_check = Arc::clone(&assistant_flag);
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    
                    let on_assistant = {
                        let assistant = assistant_check.lock().unwrap();
                        *assistant
                    };
                    
                    if !on_assistant {
                        println!("Reinjecting redirect script");
                        if let Err(e) = window.eval(REDIRECT_SCRIPT) {
                            eprintln!("Failed to reinject script: {}", e);
                        }
                    } else {
                        println!("On assistant page - script injection completely stopped");
                    }
                });
            });
            
            let continuous_window = main_window.clone();
            let loading_flag = Arc::clone(&is_loading);
            let assistant_flag = Arc::clone(&on_assistant_page);
            std::thread::spawn(move || {
                let max_iterations = 30;
                let mut i = 0;
                
                while i < max_iterations {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    
                    let on_assistant = {
                        let assistant = assistant_flag.lock().unwrap();
                        *assistant
                    };
                    
                    if on_assistant {
                        println!("Successfully reached assistant page, stopping continuous check");
                        break;
                    }
                    
                    let is_already_loading = {
                        let loading = loading_flag.lock().unwrap();
                        *loading
                    };
                    
                    if !is_already_loading {
                        println!("Reinjecting redirect script from continuous check");
                        if let Err(e) = continuous_window.eval(REDIRECT_SCRIPT) {
                            eprintln!("Failed to reinject script from continuous check: {}", e);
                        }
                    }
                    
                    i += 1;
                }
                
                println!("Continuous check completed");
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_url, 
            set_current_url, 
            go_to_assistant, 
            show_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
