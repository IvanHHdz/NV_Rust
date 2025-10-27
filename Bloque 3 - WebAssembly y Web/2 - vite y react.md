# ¡Hola Mundo!

Anteriormente, hemos visto cómo se utilizaría nuestro compilado de WebAssembly utilizando unicamente JavaScript. Sin embargo, rara vez tendremos que integrarlo de esta forma. Comúnmente lo haremos por medio de un framework. Razón por la cual ahora aprenderemos a utilizarlo con React + Vite como ejemplo, aunque el mismo proceso se puede realizar de manera relativamente similar en la mayoría de frameworks. 

Para lograr esto, primero necesitaremos crear un proyecto de Vite, que utilizará React:
```shell
# creamos el proyecto, con una plantilla de react
npm create vite@latest ejemplo -- --template react
# seguimos los pasos para crearlo

# instalamos dependencias de paso
cd ejemplo
npm install
```

En nuestro caso, lo llamaremos `ejemplo`. Y comprobamos que todo está bien corriendo el servidor:
```shell
npm run dev
```

Y abrimos la página (probablemente sea http://localhost:5173/). Debería mostrarlos la interfaz por defecto.

Ahora, podemos mover la carpeta `/pkg/` que hicimos anteriormente, para probar con el mismo ejemplo. La colocaremos dentro de `/scr/`.

Ahora, para verlo en funcionamiento, editaremos el `App.jsx`, lo dejaremos de la siguiente forma:
```javascript
// importación del wasm
// es igual que en javascript vanilla
import init, { greet } from './pkg/saludos_wasm.js';

// importaciones necesarias
import { useState, useEffect } from 'react'
import './App.css'

function App() {
    // esto será para guardar el setorno de la función
    const [message, setMessage] = useState('');
    useEffect(() => {
        // esta función carga el wasm
        async function loadWasm() {
        await init();           // lo inicia
        setMessage(greet());    // ejecuta greet() y guarda el retorno
      }
      loadWasm();               // ejecuta lo anterior
    }, []);
    return (
    <div>
      <h1>WebAssembly con Rust y Vite</h1>
      <h2>Saludando</h2>
      <p>{message}</p>
    </div>
  );
}

export default App
```

# Funciones
Podemos crear funciones que tomen parámetros:
```rust
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

y para implementarla:
```javascript
// creamos variables para los valores a sumar
const a = 13;
const b = 7;
// eso puede omitirse añadiendo los datos directamente en el código

// crea una variable para guardar los datos
const [result, setResult] = useState(null);
// ejecuta la función y guarda el resultado en result
setResult(add(a, b));
```

Y el `return` para renderizarlo:
```javascript
return (
    <div>
      <h1>WebAssembly con Rust y Vite</h1>
      <h2>Sumando</h2>
      <p>La suma de {a} + {b} = {result}</p>
    </div>
  );
```

Además de poder crear interactividad entre el JavaScript y nuestro WASM. Por ejemplo, digamos que deseamos crear dos contadores que van sumando sus valores y los van colocando en un texto de la página. Utilizaremos la misma función de antes:
```javascript
// importaciones
import { useState, useEffect } from 'react'
import './App.css'

// función app
function App() {
  // para guardar los resultados
    const [result, setResult] = useState(null);

  // para guardar los valores
    const [a, setNumero_a] = useState(1);
    const [b, setNumero_b] = useState(1);

  // esta función carga el wasm
    async function loadWasm() {
        await init();
        // realizamos una suma inicial
        setResult(add(a, b));
      }

  // ejecutamos el iniciador
    useEffect(() => {
      loadWasm();
    }, []);

  // funciones para realizar los cálculos de actualización de cada valor
    const cambioA = (event) => {
        const valor = event.target.value;
        setNumero_a(valor); // Actualiza el estado con el valor ingresado
        setResult(add(valor, b));
      };
    const cambioB = (event) => {
        const valor = event.target.value;
        setNumero_b(valor); // Actualiza el estado con el valor ingresado
        setResult(add(a, valor));
      };
      
    return (
    <div>
      <h1>WebAssembly con Rust y Vite</h1>
      <h2>Sumando</h2>
      <div>
        <label>Número a: </label>
        <!-- ejecuta la función correspondiente -->
        <input type='number' id="a" onChange={cambioA} value={a}></input>
      </div>
      <br></br>
      <div>
        <label>Número b: </label>
        <!-- ejecuta la función correspondiente -->
        <input type='number' id="b" onChange={cambioB} value={b}></input>
      </div>
      <p>La suma de {a} + {b} = {result}</p>
    </div>
    );
}

export default App
```

# Ejemplo: Ordenamiento
Para poder vizualizar de mejor manera esto, utilizaremos un ejemplo. Este ejemplo será sobre ordenar un conjunto de números:

Para lograrlo, primero debemos crear la función en Rust. Recordemos que debemos de minimizar la cantidad de veces en que pasamos datos entre rust y javascript, pues existe un coste (pequeño, pero existente) de llamar las funciones en WASM. Así que en este caso lo que haremos será pasarle toda la lista de números y devolver la lista de números:
```rust
#[wasm_bindgen]
pub fn ordenar_numeros(numeros: Vec<i32>) -> Vec<i32> {
    let lista = numeros.clone();
    // igualmente, podríamos simplemente utilizar el método .sort()
    // numeros.sort();
    ordenar(&lista)
}

fn ordenar(arreglo: & Vec<i32>) -> Vec<i32> {
    let mut lista = arreglo.clone();
    loop {
        let mut flag = false;
        for i in 0..lista.len()-1 {
            if lista[i] > lista[i+1] {
                let a = lista[i];
                lista[i] = lista[i+1];
                lista[i+1] = a;
                flag = true 
            }
        }
        if !flag {
            break
        }
    }
    lista
}
```

Como podemos ver, el algoritmo que usamos es un simple _Bubble sort_, aunque podría ser cualquier otro algoritmo de ordenamiento (incluido el algoritmo por defecto usando el método `.sort()`), pero para simplificar usaremos este. Si pasaramos arreglos mucho más grandes, lo mejor sería utilizar un algoritmo que los trabaje con rapidez.

Y una vez compilado y tras mover la carpeta generada `/pkg/` simplemente reacomodamos `App.jsx` para que muestre una lista de números y los ordene con esta función.
```javascript
// importamos la función
import init, { ordenar_numeros } from './pkg/saludos_wasm.js';

// otras importaciones necesarias
import { useState, useEffect } from 'react'
import './App.css'

// función App()
function App() {
    // para guardar los datos
    // utilizaremos 5 números, pero la función de rust para ordenarlos con número arbitrario de números
    const [a, setNumero_a] = useState(1);
    const [b, setNumero_b] = useState(1);
    const [c, setNumero_c] = useState(1);
    const [d, setNumero_d] = useState(1);
    const [e, setNumero_e] = useState(1);

    // carga el wasm
    async function loadWasm() {
        await init();
      }

    useEffect(() => {
      loadWasm();
    }, []);

    // para manejar los cambios en los inputs
    const cambio = (cambiador) => (event) => {
      const valor = Number(event.target.value);
      cambiador(valor);
    }
    
    // para cambiar todos los valores a la vez
    const cambios = (valores) => {
      const funcionesCambio = [setNumero_a, setNumero_b, setNumero_c, setNumero_d, setNumero_e];
      funcionesCambio.forEach((fn, index) => fn(valores[index]));
    };

    // para ordenar los valores
    const ordenar_wasm = () => {
      cambios(ordenar_numeros([a, b, c, d, e]));
    }
    
    return (
    <div>
      <h1>WebAssembly con Rust y Vite</h1>
      <div>
        <div>
          <label>Número a: </label>
          <input type='number' id="a" onChange={cambio(setNumero_a)} value={a}></input>
        </div>
        <br />
        <div>
          <label>Número b: </label>
          <input type='number' id="b" onChange={cambio(setNumero_b)} value={b}></input>
        </div>
        <br />
        <div>
          <label>Número c: </label>
          <input type='number' id="c" onChange={cambio(setNumero_c)} value={c}></input>
        </div>
        <br />
        <div>
          <label>Número d: </label>
          <input type='number' id="d" onChange={cambio(setNumero_d)} value={d}></input>
        </div>
        <br />
        <div>
          <label>Número e: </label>
          <input type='number' id="e" onChange={cambio(setNumero_e)} value={e}></input>
        </div>
        <br />
        <div>
          <label>Arreglo: </label>
          <span>[ {a}, {b}, {c}, {d}, {e} ]</span>
        </div>
        <br />
        <div>
          <button onClick={ordenar_wasm}>
            Ordenar Arreglo
          </button>
        </div>
      </div>
    </div>
    );
}

export default App
```