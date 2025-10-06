# WebAssembly

WebAssembly (abreviado como wasm) es un formato de código de bytes de bajo nivel que sirve como objetivo para compilar lenguajes de alto nivel como C, C++ y Rust. Su objetivo es permitir la ejecución de aplicaciones en la web y otras plataformas con una velocidad cercana a la nativa, ofreciendo un rendimiento y una versatilidad significativamente mejorados en comparación con tecnologías anteriores. 

Para lo que veremos en este curso, necesitaremos:
- Rust (`rustup`, `rustc` y `cargo`)
- `wasm-pack` y `cargo-generate`, que se pueden instalar por medio de `cargo`.
- npm

Si tiene errores para compilar, verifique la instalación del `wasm-pack`. Si falla por cargo, vuelva a intentarlo por npm con:
```shell
npm install -g wasm-pack
```

Si falla, trate instalando Rustup con:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Recuerde actuzalizar la terminal. Al terminar la instalación del Rustup se indica cómo.

# ¡Hola Mundo!

Para crear un "Hola Mundo" con wasm primero deberemos crear un projecto de cargo. Este caso será una librería:
```shell
cargo init saludos_wasm --lib
```

En este caso la llamaremos `saludos_wasm`. Vamos a modificar el `./src/lib.rs` para que esté solo una función que salude:
```rust
use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen]
pub fn greet() -> String {
    web_sys::console::log_1(&"¡Hola, mundo desde WebAssembly!".into());
    format!("¡Hola, mundo desde WebAssembly!")
}
}
```
Esta función imprimirá en la consola del navegador `"¡Hola, mundo desde WebAssembly!"`, además de retornarlo como cadena de texto al llamarla.

Y ahora modificamos el `Cargo.toml` para tener los crates que necesitamos. Además de otras cosas necesarias:
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.104"
web-sys = { version = "0.3.81", features = ["console"] }

[dev-dependencies]
wasm-bindgen-test = "0.3.54"

[profile.release]
opt-level = "s"
```

Y pasaríamos a compilar utilizando:
```shell
wasm-pack build --target web
```

Una vez terminado nos creará una carpeta `./pkg/` con todo lo necesario.

Ahora lo que haremos será integrarlo directamente a una pequeña página web utilizando JavaScript. Para ello, crearemos dos archivos: `index.html`, que es el código de la página; e `index.js` que será el código JavaScript que ejecutará la misma.

En el `index.html` crearemos una página sencilla:
```html
<!DOCTYPE html>
<html lang="es">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Rust WASM con JavaScript Vanilla</title>
    </head>
    <body>
        <h1>Usando Rust WASM</h1>

        <!-- se crea el botón y se le asigna la función greet -->
        <button onclick="greet()">Saludar</button>

        <!-- aquí meteremos el retorno de la función -->
        <p id="result"></p>

        <!-- importación del módulo WASM -->
        <script type="module" src="./index.js"></script>
    </body>
</html>
```

Y el `index.js` lo dejaremos de la siguiente manera:
```javascript
// importación del módulo WASM
import init, { greet } from './pkg/saludos_wasm.js';

// Función para inicializar el módulo WASM
async function run() {
    await init(); // Inicializa el módulo WASM
}

// Ejecuta la inicialización al cargar la página
run();

// Creación de la función global para el botón
window.greet = function() {
    const result = greet(); // Llama a la función Rust
    document.getElementById('result').textContent = result;
};
```

Y meteremos todo dentro de una misma carpeta, en este caso la llamaremos `web`. Esa carpeta debe tener tando el `index.html` como el `index.js`, también meteremos la carpeta `/pkg/` que compilamos.

Por último, necesitaremos ejecutar un servidor. Podemos hacerlo de manera sencilla usando el siguiente comando dentro de la carpeta `/web/`:
```shell
http-server -p 8000
```

Si no tenemos instalado `http-server`, podemos instalarlo usando `npm` con:
```shell
npm install -g http-server
```

Ahora, si abrimos la página, veremos un botón que dice `Saludar`, si lo presionamos nos aparecerá abajo `¡Hola, mundo desde WebAssembly!` (tal como lo configuramos). Y si abrimos la consola, veremos que también imprime `"¡Hola, mundo desde WebAssembly!"`.