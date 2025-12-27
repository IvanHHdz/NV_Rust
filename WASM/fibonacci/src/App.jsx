import init, { fibonacci } from './pkg/funciones.js'
import { fibonacci_js } from './funciones_js.js' 

import { useState, useEffect } from 'react'
import './App.css'

function App() {
  const [result1, setResult1] = useState(1);
  const [result2, setResult2] = useState(1);
  
  const [time1, setTime1] = useState(1);
  const [time2, setTime2] = useState(1);

  const [n, setNumero] = useState(1);

  async function loadwasm() {
    await init();
  }

  useEffect(() => {
    loadwasm();
  }, [])

  const ejemplo = (event) => {
    const valor = event.target.value;
    setNumero(valor);
    
    let start = performance.now();
    setResult1(fibonacci(valor));
    let end = performance.now();
    setTime1(end - start);
    
    start = performance.now();
    setResult2(fibonacci_js(valor));
    end = performance.now();
    setTime2(end - start);
  };

  return (
    <>
      <h1>
        Sucesion de Fibonacci con JS vs WASM
      </h1>
      <div>
        <label>
          Valor:<br></br>
        </label>
        <input type='number' id='n' onChange={ejemplo} value={n}></input>
      </div>
      <div>
        <p>
          WASM = {result1} en {time1} ms
        </p>
        <p>
          JS = {result2} en {time2} ms
        </p>
      </div>
    </>
  )
}

export default App
