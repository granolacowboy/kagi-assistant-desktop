function App() {
  return (
    <main className="container">
      <iframe
        src="https://kagi.com/assistant"
        title="Kagi Assistant"
        className={`kagi-webview`}
      />
    </main>
  );
}

export default App;
