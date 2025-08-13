# Rust

Rust es un lenguaje de programación compilado multiparadigma y de propósito general. 

Inició como un proyecto personal de Graydon Hoare, empleado de Mozilla. A partir de 2009 es patrocinado por Mozilla, siguiendo sus políticas de desarrollo abierto. Actualmente el proyecto es dirigido por la Rust Foundation.

Desde 2020 es uno de los lenguajes más usados en criptografía y en 2022 se convirtió en el tercer lenguaje en ser usado dentro del kernel de Linux, junto a C y Assembly.

Posee una sintaxis similar a C++, trabaja a un nivel similar a C, toma cierta inspiración de Haskell y trabaja con multiprogramación como Go.

Destaca por su manejo seguro de la memoria sin necesidad de un recolector de basura, su concurrencia segura y eficiente, y su rendimiento similar a C y C++.

Como dato extra: Los usuarios de Rust se refieren a sí mismos como rustáceos (Rustaceans) y usan a Ferris como su mascota.

Conozca a Ferris:

![Ferris](https://upload.wikimedia.org/wikipedia/commons/thumb/2/20/Rustacean-orig-noshadow.svg/500px-Rustacean-orig-noshadow.svg.png)

> Es importante remarcar que para este curso se asume que el alumno **ya sabe programar** en algún lenguaje. Es decir ya conoce **al menos un lenguaje de programación**. Razón por la cual se harán comparaciones para ejemplificar y explicar de mejor manera, especialmente comparaciones con **C++** y **Python**.

# Instalación

Primero que todo, debemos instalar el Rust en nuestro computador.

En Windows, podemos descargar el instalador desde [el sitio oficial de rust](https://rustup.rs/). Esto instalará automáticamente todo lo que necesitaremos.

Si estamos desde estamos desde Linux o MacOS, podemos simplemente instalarlo desde la consola con:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Lo cual nos instalará todo.

Para verificar que todo funcionó, podemos usar el siguiente comando (en Powershell de Windows o en la terminal de Linux o MacOS):
```shell
rustc --version
```

Y nos debería mostrar algo como:
```
rustc 1.89.0 (29483883e 2025-08-04)
```

Si en un dado caso, por alguna razón, después de instalar `rustc` no reconoce el comando anterior, verifique el `PATH`.

# Compilador: `rustc` y `cargo`

Rust posee dos compiladores, ambos se instalan por defecto al instalar Rust de la forma que vimos anteriormente. Estos compiladores son `rustc` el compilador por defecto y `cargo` que es un compilador y administrador de paquetes.

La ventaja de utilizar `cargo` por sobre `rustc` está en su capacidad de manejo de paquetes, aunque para proyectos sencillos `rustc` es suficiente.

# IDEs

Rust, a diferencia de Python, no nos instalará un IDE por defecto. Afortunadamente existen muchas opciones:

| IDE / Editor         | Pros                                                | Contras                                           | Nivel recomendado       |
|---------------------|----------------------------------------------------|-------------------------------------------------|-----------------------|
| **VS Code**          | Ligero, extensible, rust-analyzer para autocompletado y análisis en tiempo real | Requiere configurar extensiones para sacarle todo el provecho | Principiante a avanzado |
| **IntelliJ IDEA / CLion** | Integración completa, depuración, refactorización avanzada, soporte para proyectos grandes | CLion requiere licencia paga; más pesado que VS Code | Intermedio a avanzado |
| **Eclipse Corrosion** | Útil si ya usas Eclipse; soporte básico de Rust | Menos pulido, menos extensiones y comunidad pequeña | Principiante a intermedio |
| **Emacs / Vim / Neovim** | Extremadamente personalizable, muy rápido, integración con rust-analyzer | Configuración inicial compleja; curva de aprendizaje alta | Intermedio a avanzado |
| **Rust Playground (online)** | No requiere instalación, útil para pruebas rápidas | Limitado para proyectos grandes; sin depuración local | Principiante |

En este curso, vamos a usar VS Code.

# Hello World!

Para asegurarnos que los compiladores funcionan de manera correcta, vamos a crear el programa básico para iniciar a aprender un lenguaje de programación: un Hola Mundo.

En Rust, el código de un "Hola Mundo" es el siguiente:
```rust
fn main(){
    println!("Hello World!");
}
```

Como puede ver, el código tiene ciertas similitudes con Python. Por ejemplo, la función que imprimirá en pantalla se llama `println!()` (aunque no es una función, sino una macro), similar al `print()`, la función `main()` se declara con `fn` similar a como se declaran en Python con `def`. Y tiene delimitadores de bloques con `{}` como en C++ al igual que el `;` al final de la línea.

Así que seguramente puede inferir cómo funcionan esas líneas: crea una función llamada `main()` que será la que se ejecutará al correr el programa, dentro ejecutamos la función (recuerde que no es una función, sino un macro, pero la llamaremos función por ahora para simplificar) que imprimirá en pantalla la cadena de texto que pasamos como parámetro, en este caso `"Hello World!"`.

Para ejecutar esto, debemos compilarlo (así como se hace en C++), y como anteriormente mencionamos, tenemos dos compiladores. 

## `rustc`

Podemos crear un archivo con el nombre que queramos y la terminación `.rs` (para este ejemplo, será `hello_world.rs`) y compilarlo abriendo la consola (la terminal o el powershell) en la carpeta que guardamos el archivo y ejecutando el siguiente comando:

```shell
rustc hello_world.rs
```

> Nota: Podemos abrir la terminal desde VS Code haciendo click izquierdo sobre la carpeta y abrir en terminal. También podemos usar el atajo
> ```
> Ctrl + \`


Lo que nos compilará el ejecutable. Este lo podemos ejecutar directamente desde la consola con:

(Windows)
```shell
.\hello_world
```
(Linux/MacOS)
```shell
./hello_world
```

Y esto nos mostrará en pantalla:
```
Hello World!
```

## Cargo

Desde `cargo`, podemos crear un proyecto con (vamos a llamarlo `helloworld` ahora):
```shell
cargo new helloworld
```
Esto nos creará un proyecto completo. Dentro de la carpeta `helloworld > src > main.rs` estará el código fuente, que podemos editar. Aunque al crear un proyecto nos creará automáticamente el código de un "Hola Mundo".

Y nos movemos dentro de la carpeta con:
```shell
cd helloworld
```

Ahora, solo debemos ejecutar:
```shell
cargo run
```

Y se compilará y ejecutará nuestro programa. Podemos encontrar el ejecutable en `helloworld > target > debug > helloworld.exe`.

Al hacer el `cargo run`, como mencionamos anteriormente, el programa se ejecutará instantáneamente, mostrandonos:
```
Hello, World!
```

Si todo ha salido bien, ¡Felicidades, ha creado su primer programa en Rust!

![Ferris Feliz](https://rustacean.net/assets/rustacean-flat-happy.svg)

> ¡Ferris está contento por su logro!