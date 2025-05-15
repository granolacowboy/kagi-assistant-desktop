import { useState, useEffect } from "react";
import "./App.css";

function App() {
  const [isLoading, setIsLoading] = useState(true);
  const [redirectAttempted, setRedirectAttempted] = useState(false);
  const [redirectTimer, setRedirectTimer] = useState<number | null>(null);
  const [countdown, setCountdown] = useState(5);
  
  const goToAssistant = () => {
    console.log("MANUALLY NAVIGATING TO ASSISTANT");
    try {
      localStorage.setItem('kagiLoggedIn', 'true');
    } catch (e) {
      console.error(`Error saving login state: ${e}`);
    }
    window.location.href = 'https://kagi.com/assistant';
  };

  useEffect(() => {
    console.log("====== KAGI ASSISTANT DEBUG ======");
    console.log("App starting. Going to check login state.");
    
    const startRedirectCountdown = () => {
      if (redirectTimer !== null) return;
      
      setRedirectAttempted(true);
      console.log("STARTING REDIRECT COUNTDOWN (5 seconds)");
      
      const timer = window.setInterval(() => {
        setCountdown(prev => {
          const newCount = prev - 1;
          console.log(`Redirecting to assistant in ${newCount} seconds...`);
          
          if (newCount <= 0) {
            clearInterval(timer);
            console.log("REDIRECTING NOW");
            goToAssistant();
          }
          return newCount;
        });
      }, 1000);
      
      setRedirectTimer(timer as unknown as number);
    };
    
    try {
      const hasLoggedInBefore = localStorage.getItem('kagiLoggedIn') === 'true';
      console.log(`Previously logged in: ${hasLoggedInBefore ? 'YES' : 'NO'}`);
      
      if (hasLoggedInBefore && !redirectAttempted) {
        console.log("Previous login detected, will redirect to assistant shortly");
        startRedirectCountdown();
      }
    } catch (e) {
      console.error(`Local storage error: ${e}`);
    }
    
    const checkLocation = () => {
      const currentUrl = window.location.href;
      console.log(`Current URL: ${currentUrl}`);
      
      if (!redirectAttempted && (
          currentUrl === 'https://kagi.com' || 
          currentUrl === 'https://kagi.com/' || 
          currentUrl.startsWith('https://kagi.com/?') ||
          currentUrl.startsWith('https://kagi.com/search')
        )) {
        console.log("DETECTED KAGI HOMEPAGE - User is logged in");
        try {
          localStorage.setItem('kagiLoggedIn', 'true');
        } catch (e) {
          console.error(`Failed to save login state: ${e}`);
        }
        
        startRedirectCountdown();
      }
    };
    
    setTimeout(() => {
      setIsLoading(false);
      console.log("Loading spinner hidden");
    }, 2000);
    
    setTimeout(checkLocation, 1000);
    
    const locationInterval = setInterval(checkLocation, 2000);
    
    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.key === 'a' && e.ctrlKey) {
        console.log("MANUAL OVERRIDE: Ctrl+A pressed");
        goToAssistant();
      }
    };
    
    window.addEventListener('keydown', handleKeyPress);
    
    return () => {
      clearInterval(locationInterval);
      if (redirectTimer !== null) {
        clearInterval(redirectTimer);
      }
      window.removeEventListener('keydown', handleKeyPress);
    };
  }, [redirectAttempted, redirectTimer]);

  return (
    <main className="container">
      {isLoading && (
        <div className="loading">
          <div className="spinner"></div>
          <h2>Loading Kagi...</h2>
        </div>
      )}
      
      <iframe 
        src="https://kagi.com/signin" 
        title="Kagi" 
        className={`kagi-webview ${isLoading ? 'hidden' : ''}`}
      />
      
      <button 
        onClick={goToAssistant}
        style={{
          position: 'fixed',
          bottom: '20px',
          right: '20px',
          zIndex: 99999,
          backgroundColor: 'red',
          color: 'white',
          fontWeight: 'bold',
          padding: '15px 20px',
          borderRadius: '8px',
          fontSize: '16px',
          cursor: 'pointer'
        }}
      >
        GO TO ASSISTANT NOW
      </button>
      
      {redirectAttempted && countdown > 0 && (
        <div style={{
          position: 'fixed',
          top: '20px',
          left: '20px',
          zIndex: 99999,
          backgroundColor: 'rgba(0,0,0,0.8)',
          color: 'white',
          padding: '10px 15px',
          borderRadius: '5px',
          fontSize: '14px'
        }}>
          Redirecting to Assistant in {countdown}s...
        </div>
      )}
    </main>
  );
}

export default App;
