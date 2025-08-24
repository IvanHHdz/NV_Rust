# Paquetes, crates y módulos

A medida que nuestros programas crecen nos veremos en la necesidad de organizarlo. Para esto, tendremos que separar o modularizar las partes de nuestro programa que realizan tal o cual acción. Para ayudarnos a organizar nuestro código Rust nos provee de una serie de características, comunmente apodadas como _module system_ que incluye:
- Paquetes o _Packages_
- Cajas o _Crates_
- Módulos y _use_
- Rutas o _Paths_

## Crates
Son la unidad mínima que considera el compilador. Incluso cuando compilamos un solo archivo para el compilador de Rust (`rustc` o `cargo`) se trata de una crate.

De estas existen dos tipos: _binary crates_ y _library crates_. Las primeras son programas, que se compilan como ejecutables. Que deben tener su función `main()` mediante la cual serán ejecutados.

Las segundas, las _library crates_ no tienen una función `main()` y no se compilan como ejecutables. En su lugar contienen funcionalidades con la intención de compartir estas entre programas. Un ejemplo es `rand`, que [usamos al principio del curso](../1.1%20-%20Conceptos%20básicos/ConceptosBásicos.md), la cual posee funcionalidades para generar números aleatorios.

## Paquetes
Un paquete está construido por uno o varios crates, y contiene un archivo `Cargo.toml` que especifíca cómo compilar dichos crates.

Podemos compilarlas sencillamente por medio de Cargo. Al iniciar un nuevo proyecto en cargo estamos creando, en sí, un paquete.

Creamos un nuevo proyecto con:
```shell
cargo new my-project
```

## Módulos
Creamos módulos utilizando `mod`, seguido del nombre del módulo. Podemos declarar módulos dentro de módulos y así sucesivamente. Pero hay que tener en cuenta que los módulos que creemos dentro de otros módulos serán privados por defecto para sus módulos padres. Para hacerlos públicos utilizamos `pub mod` en lugar de `mod`. Lo mismo ocurre con funciones, estructuras, emuns, etc. Para poder utilizar cosas que estén declaradas dentro de un módulo necesitamos llamarlo con `use`, así como hemos llamado otras funciones de la librería estándar (como hicimos, por ejemplo, con los [Hash Maps](../1.5%20-%20Datos%20compuestos/DatosCompuestos.md)).
```rust
// ejemplo llamando
use crate::jardin::regar_el_jardin;

// ejemplo declarando
mod jardin;
```
Para encontrar el lugar donde se encuentran las declaraciones de lo que llamamos, Cargo buscará primero en las llaves que podemos colocar en lugar del `;`. Si no encuetra nada buscará un archivo del mismo nombre (en el ejemplo que mostramos antes buscaría un `./jardin.rs`) en la misma carpeta. Si no, buscará una carpeta con ese nombre que contenga un archivo `mod.rs` (en nuestro caso `./jardin/mod.rs`).

En el ejemplo anterior, podríamos tenerlo de la siguiente forma:
```
casa                <- nombre del proyecto
├── Cargo.lock      
├── Cargo.toml
└── src             <- carpeta con archivos fuente
    ├── jardin      <- carpeta del módulo
    │   └── mod.rs  <- archivo del módulo
    └── main.rs     <- archivo raíz

```

De esta misma forma podemos tener también multiples módulos dentro de otro módulo para agruparse. Dónde algunos pueden ser públicos y otros no.

A estas rutas que utilizamos para traer o llamar cierta párte de un módulo (o el módulo completo) se les llama rutas (paths). Las mismas pueden ser relativas o absolutas, y se separan con `::`. Funcionan similar a como se ordenan carpetas. La ruta absoluta sería desde la raíz (`C:\` en Windows o `/` en Linux) hasta donde quisieramos llegar. Por otro lado, la ruta relativa es partir desde el lugar donde estamos, usando precisamente el nombre del módulo en el que lo llamamos (similar a usar `./`). 

De igual forma, tenemos forma de utilizar módulos que estén "sobre" el que lo llama. Esto, siguiendo con el ejemplo de las carpetas, sería como tuvieramos que retroceder carpetas para alcanzar la que buscamos (similar a usar `../`).

Claro que para que sea accesible debe ser público explícitamente, como habíamos mencionado. Y deben ser públicos tanto el módulo que queremos acceder como todos sus módulos padres que no compartamos.

Y al igual que en otros lenguajes, debemos hacer pública las variables individuales de una estructura para poderla modificar. Las variantes de un enum serán públicas (todas) si el propio enum es público.

Y de igual forma que podemos traer una función, una estructura o un enum usando `use`, podemos hacerlo con el módulo completo. Esto lo trae **únicamente al scope o alcance que en el que estemos**. A menos que hagamos dicha importación pública con `pub use`.

También podemos agrupar importaciones, si tenemos muchas. Por ejemplo:
```rust
use std::cmp::Ordering;
use std::io;
```
Podemos reescribirlo como:
```rust
use std::{cmp::Ordering, io};
```

Y así como en otros lenguajes podemos importar **todo** utilizando `*`:
```rust
use std::collections::*;
```

## Apodos
Al igual que otros lenguajes como Python, podemos utilizar `as` para renombrar importaciones, para facilitar su uso o para que sea más sencillo entender lo que hace.

# Manejo de errores

Cap. 9

# Tipos de datos genéricos
Cap. 10.1

# Traits
Cap. 10.2

# Lifetime
Cap. 10.3

# Pruebas (`test`)
Cap. 11