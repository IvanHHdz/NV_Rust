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