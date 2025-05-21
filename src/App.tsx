import { useState, useEffect } from "react";
import "./App.css";

function App() {
  const [isLoading, setIsLoading] = useState(true);

  const goToAssistant = () => {
    try {
      localStorage.setItem('kagiManualRedirect', 'true'); 
    } catch (e) {
    }
    window.location.href = 'https://kagi.com/assistant';
  };

  useEffect(() => {
    const loadingTimer = setTimeout(() => {
      setIsLoading(false);
    }, 2500);

    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.key === 'a' && e.ctrlKey && e.shiftKey) {
        goToAssistant();
      }
    };

    window.addEventListener('keydown', handleKeyPress);

    return () => {
      clearTimeout(loadingTimer);
      window.removeEventListener('keydown', handleKeyPress);
    };
  }, []);

  return (
    <main className="container">
      {isLoading && (
        <div className="loading">
          <div className="spinner"></div>
          <h2>Loading Kagi Assistant...</h2>
        </div>
      )}

      <iframe
        src="https://kagi.com/assistant"
        title="Kagi Assistant"
        className={`kagi-webview ${isLoading ? 'hidden' : ''}`}
        onLoad={() => {
          setIsLoading(false);
        }}
      />

      {!isLoading && (
        <button
          onClick={goToAssistant}
          style={{
            position: 'fixed',
            bottom: '20px',
            right: '20px',
            zIndex: 99999,
            backgroundColor: '#007bff',
            color: 'white',
            fontWeight: 'bold',
            padding: '10px 20px',
            borderRadius: '5px',
            fontSize: '14px',
            cursor: 'pointer',
            border: 'none',
            boxShadow: '0 2px 5px rgba(0,0,0,0.2)'
          }}
        >
          Go To Assistant
        </button>
      )}
    </main>
  );
}

export default App;
