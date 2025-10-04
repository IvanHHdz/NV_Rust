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

#[wasm_bindgen]
pub fn greet() {
    println!("¡Hola, mundo desde WebAssembly!");
}
```

Y ahora modificamos el `Cargo.toml` para tener los crates que necesitamos. Además de otras cosas necesarias:
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.84"

[dev-dependencies]
wasm-bindgen-test = "0.3.34"

[profile.release]
opt-level = "s"
```

Y pasaríamos a compilar utilizando:
```shell
wasm-pack build
```

Una vez terminado nos creará una carpeta `./pkg/` con todo lo necesario.

<!-- TODO verifica si mejor lo podemos hacer directamente con react o vite
Ahora para probarlo vamos a crear un proyecto al que llamaremos prueba con:
```shell
npm init wasm-app prueba
```

Nos iremos a `./prueba/package.json` y agregaremos esto:
```json
  "homepage": "https://github.com/rustwasm/create-wasm-app#readme",
  "dependencies": {
    "saludo_wasm": "file:../pkg"
  },
  "devDependencies": {
```
Como podemos ver, lo agregamos entre `"homepage"` y `"devDependencies"`.
 TODO verificar si eso es necesario 

Y modificamos el `./prueba/index.js` para que importe nuestro wasm y ejecute nuestra función:
```javascript
import * as wasm from "saludo_wasm";

wasm.greet();
```

Y ejecutamos el servidor con:
```shell
npm run start
```
-->
