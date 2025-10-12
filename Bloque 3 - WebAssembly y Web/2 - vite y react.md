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
      <p>{message}</p>
    </div>
  );
}

export default App
```

# El juego de la vida

Ahora que ya aprendimos las nociones básicas, es momento de entrar en detalle con algo un poco más complejo: crearemos una simulación de **El Juego de la Vida**, diseñado originalmente por _Conway_.
> Nota: si casulamente no tiene ni idea, o no recuerda bien qué es o en qué consiste este algoritmo, puede verificar la información proporcionada en [Wikipedia](https://es.wikipedia.org/wiki/Juego_de_la_vida), donde puede encontrar mayor información de este interesante tema.

Las reglas son sencilla, pues es un juego de 0 jugadores. Todas las células se actualizan simultáneamente en cada turno, siguiendo estas reglas: :
- Nace: Si una célula muerta tiene exactamente 3 células vecinas vivas "nace" (es decir, al turno siguiente estará viva).
- Muere: una célula viva puede morir por uno de 2 casos:
  - Sobrepoblación: si tiene más de tres vecinos alrededor.
  - Aislamiento: si tiene solo un vecino alrededor o ninguno.
- Vive: una célula se mantiene viva si tiene 2 o 3 vecinos a su alrededor.

Originalmente, el juego fue pensado para jugarse en un universo (una cuadrícula) infinita. Sin embargo, debido a nuestras limitaciones de memoria no infinita y poder computacional no infinito, no podemos hacer eso. Así que, para nuestro caso, haremos un universo "circular", por así decirlo, donde los bordes llevan al otro lado del universo.

