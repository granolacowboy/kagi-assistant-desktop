import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Listen for window load event 
    const handleLoad = () => {
      setTimeout(() => {
        setIsLoading(false);
      }, 2000); // Give the window a bit of time to fully render
    };

    window.addEventListener('load', handleLoad);

    // Set up a listener to handle external links
    const handleMessage = async (event: MessageEvent) => {
      if (event.data && typeof event.data === 'object') {
        // If it's a link click event
        if (event.data.type === 'link-click' && event.data.url) {
          await invoke('send_url', { url: event.data.url });
          window.open(event.data.url, '_blank');
        }
        // If it's a navigation event
        else if (event.data.type === 'navigation' && event.data.url) {
          await invoke('set_current_url', { url: event.data.url });
        }
      }
    };

    window.addEventListener('message', handleMessage);

    // Inject scripts to capture current URL and link clicks
    const injectScript = () => {
      try {
        // Report current URL and listen for changes
        const script = document.createElement('script');
        script.textContent = `
          // Report the initial URL
          window.parent.postMessage({ 
            type: 'navigation', 
            url: window.location.href 
          }, '*');
          
          // Listen for navigation events
          const pushState = history.pushState;
          const replaceState = history.replaceState;
          
          history.pushState = function() {
            pushState.apply(history, arguments);
            window.parent.postMessage({ 
              type: 'navigation', 
              url: window.location.href 
            }, '*');
          };
          
          history.replaceState = function() {
            replaceState.apply(history, arguments);
            window.parent.postMessage({ 
              type: 'navigation', 
              url: window.location.href 
            }, '*');
          };
          
          window.addEventListener('popstate', function() {
            window.parent.postMessage({ 
              type: 'navigation', 
              url: window.location.href 
            }, '*');
          });
          
          // Capture link clicks
          document.addEventListener('click', (event) => {
            const link = event.target.closest('a');
            if (link && link.href && link.target === '_blank') {
              event.preventDefault();
              window.parent.postMessage({ 
                type: 'link-click', 
                url: link.href 
              }, '*');
            }
          });
        `;
        document.head.appendChild(script);
      } catch (error) {
        console.error('Failed to inject script:', error);
      }
    };

    // Run the script injection after a short delay
    setTimeout(injectScript, 1000);
    
    return () => {
      window.removeEventListener('load', handleLoad);
      window.removeEventListener('message', handleMessage);
    };
  }, []);

  return (
    <main className="container">
      {isLoading && (
        <div className="loading">
          <div className="spinner"></div>
          <h2>Loading Kagi...</h2>
        </div>
      )}
    </main>
  );
}

export default App;
