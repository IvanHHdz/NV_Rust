import init, { greet } from './pkg/saludos_wasm.js';

import { useState, useEffect } from 'react'
import './App.css'

function App() {
    const [message, setMessage] = useState('');
    useEffect(() => {
        async function loadWasm() {
        await init();
        setMessage(greet());
      }
      loadWasm();
    }, []);
    return (
    <div>
      <h1>WebAssembly con Rust y Vite</h1>
      <p>{message}</p>
    </div>
  );
}

export default App
