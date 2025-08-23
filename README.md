# NV/Rust

Bienvenido al curso de Rust desde 0. En este curso vamos a partir desde lo más básico y luego especializarnos en ciertas áreas más específicas para poder ver cómo podemos aprovechar las ventajas que posee Rust en ciertos entornos.

Lo recomendable, es iniciar con el primer bloque, y luego elegir el bloque que le interese más.

Es recomendable también ya saber programar en algún lenguaje de programación antes de tomar el curso, pues en el curso se asume que se sabe algún lenguaje de antemano.

# [Bloque 1: Fundamentos de Rust](./Bloque%201%20-%20Fundamentos%20de%20Rust/)

## [Introducción](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.0%20-%20Introducción/Introducción.md)
1. Rust
2. Instalación
3. Compilador: `rustc` y `cargo`
4. IDEs
5. Hello World!

## [Conceptos básicos](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.1%20-%20Conceptos%20básicos/ConceptosBásicos.md)
1. Variables y mutabilidad
2. Tipos de datos
3. Operaciones
4. Input/Output
5. Importaciones
6. `const`

## [Control de flujo](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.2%20-%20Control%20de%20flujo/ControlDeFlujo.md)
1. Condicionales
2. Bucles

## [Funciones y bloques](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.3%20-%20Funciones%20y%20Bloques/FuncionesYBloques.md)
1. Funciones
2. Parámetros y retorno
3. Bloques y alcance

## [_Ownership_](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.4%20-%20Ownership/Ownership.md)
1. Reglas de propiedad
2. Cadenas de texto
3. Mover
4. Clonar y copiar
5. Funciones
6. Referencias y _borrowing_
7. El compilador

## [Datos compuestos](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.5%20-%20Datos%20compuestos/DatosCompuestos.md)
1. Cadenas de Texto
2. Arreglos
3. Tuplas
4. Vectores
5. Hash Maps
6. `slice`

## [Estructuras y Enums](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.6%20-%20Estructuras%20y%20Enums/EstructurasYEnums.md)
1. Estructuras
2. Métodos
3. Enums
4. Match
5. `if let`

## [Conceptos medios](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.7%20-%20Conceptos%20medios/ConceptosMedios.md)
1. Paquetes, `crates` y módulos
2. Manejo de errores
3. Tipos de datos genéricos
4. Traits
5. Lifetime
6. Pruebas (`test`)

## [Concurrencia y asincronía](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.8%20-%20Concurrencia%20y%20asincronía/ConcurrenciaAsincronia.md)
1. Hilos
2. Transferencia de información
3. Programación asincrónica
4. Macros

## [Conceptos avanzados](./Bloque%201%20-%20Fundamentos%20de%20Rust/1.9%20-%20Conceptos%20avanzados/ConceptosAvanzados.md)
1. Programación Funcional
2. Programación Orientada a Objetos
3. Punteros inteligentes
4. `unsafe`
5. Macros

# Bloque 2: Sistemas Embebidos y Críticos  
*(Enfocado en bajo nivel, control de hardware y aplicaciones de tiempo real)*  
## Entorno para embebidos
1. `no_std` y ambientes sin OS  
2. Target-specific compilation (ARM Cortex-M, RISC-V)  

## Gestión de memoria avanzada  
1. Allocators personalizados  
2. Memory-mapped I/O  

## Concurrencia en sistemas críticos 
1. Interrupciones y manejadores (ISRs)  
2. Sincronización atómica  

## Abstracciones seguras de hardware  
1. PAC (Peripheral Access Crates)  
2. HAL (Hardware Abstraction Layer)  

## Gestión de energía y optimización
1. Low-power modes  
2. Técnicas de optimización para recursos limitados  

## Herramientas específicas
1. `probe-rs`, `cargo-embed`  
2. Debugging con GDB/OpenOCD  

# Bloque 3: Criptografía y Blockchain  
*(Uso de Rust para aplicaciones seguras y descentralizadas)*  

## Fundamentos criptográficos  
1. Primitivas: hashing (SHA, BLAKE3), cifrado simétrico (AES) 
2. Criptografía de curva elíptica (ECC, Ed25519)  

## Librerías clave  
1. `ring`, `openssl`, `libsodium` bindings  
2. `arkworks` para zk-SNARKs  

## Blockchain fundamentals  
1. Estructuras de datos: Merkle Trees, Patricia Tries  
2. Consensos (PoW, PoS, PBFT)  

## Desarrollo de Smart Contracts  
1. Entornos: Solana (Anchor Framework), Polkadot (Substrate)  
2. WASM en blockchains  

## Seguridad avanzada
1. Side-channel attacks mitigation  
2. Formal verification (usando `kani`)  

# Bloque 4: WebAssembly, Servidores y Backend  
*(Rust para aplicaciones web y servicios escalables)*  

## WebAssembly (WASM)  
1. Compilación a WASM: `wasm-pack`, `wasm-bindgen` 
2. Interoperabilidad JS/Rust  
3. Frameworks: Yew, Leptos  

## APIs y Servidores Web  
1. HTTP servers: Axum, Actix Web, Rocket  
2. Protocolos: REST, gRPC (con `tonic`)  

## Concurrencia en backend  
1. Async/Await avanzado  
2. Gestión de conexiones: Connection pooling  

## Persistencia de datos  
1. ORMs: Diesel, SeaORM  
2. NoSQL: Redis, MongoDB (con crates oficiales)  

## Despliegue y escalabilidad
1. Containers (Docker)  
2. Serverless (AWS Lambda, Cloudflare Workers)  

## Ecosistema Cloud
1. Integración con Kubernetes  
2. Observabilidad: Logging, métricas (OpenTelemetry)  
