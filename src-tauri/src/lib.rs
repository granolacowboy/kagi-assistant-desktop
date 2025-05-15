use tauri::{Manager, Window};

#[tauri::command]
fn send_url(url: &str) -> String {
    format!("URL received: {}", url)
}

#[tauri::command]
async fn show_window(window: Window) {
    if let Err(e) = window.show() {
        eprintln!("Failed to show window: {}", e);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![send_url, show_window])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            
            let window_clone = main_window.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(3));
                
                if let Err(e) = window_clone.show() {
                    eprintln!("Failed to show window: {}", e);
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
