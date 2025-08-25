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

Una parte fundamental de desarrollar aplicaciones son los errores. Como mencionabamos [antes](../1.4%20-%20Ownership/Ownership.md), el compilador no solo es muy estricto, sino que es nuestro mejor amigo a la hora de buscar errores. Muchos errores se evitan gracias al compilador. Lastimosamente, muchos no es todos. Como en otros lenguajes de programación, se debe tener una forma de manejar estos errores. 

En muchos lenguajes se tiene una forma sencilla de abordar los errores: excepciones. Cosas como bloques `try-catch` o `try-except` son muy usadas en otros lenguajes para manejar los errores de todo tipo. Sin embargo, en Rust no es así, pues para comenzar: en Rust no existen las excepciones.

Rust reconoce dos tipos de errores: errores de los que se puede recuperar, y errores de los que no se puede recuperar (_recoverable_ y _unrecoverable_ respectivamente). Rust posee un tipo (`Result<T, E>`) para los que son recuperables y una macro (`panic!()`) para los que no son recuperables.

## Errores irrecuperables

En ocasiones nuestro programa puede llegar a fallar de forma tal que no hay nada que podeamos hacer para solucionarlo. Para tales casos Rust tiene el macro `panic!()`.

```rust
fn main() {
    panic!("*se crashea*");
}
```

Y si lo ejecutamos:
```
❯ ./test

thread 'main' panicked at test.rs:2:5:
*se crashea*
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Este mensaje nos indica dónde crasheó el programa (en inglés suelen utilizar el término _panicked_). El `test.rs:2:5` indica la parte exacta: archivo llamado `test.rs`, línea `2`, carácter `5`. Con el mensaje de error `"*se crashea*"`. Y si corremos el siguiente comando podremos ver más información que nos ayudará a saber qué salió mal:
```shell
RUST_BACKTRACE=1 cargo run
```

Usando cargo. 
```
thread 'main' panicked at src/main.rs:2:5:
*se crashea*
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /builddir/build/BUILD/rust-1.89.0-build/rustc-1.89.0-src/library/std/src/panicking.rs:697:5
   1: core::panicking::panic_fmt
             at /builddir/build/BUILD/rust-1.89.0-build/rustc-1.89.0-src/library/core/src/panicking.rs:75:14
   2: tests::main
             at ./src/main.rs:2:5
   3: core::ops::function::FnOnce::call_once
             at /builddir/build/BUILD/rust-1.89.0-build/rustc-1.89.0-src/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

## Errores recuperables

Pero claro, lo mejor es tratar los errores de otra forma que no sea viéndolos crashear. Además, no todos los errores son tan fatales como para que el programa tenga que crashear si ocurren.

Para estos casos, Rust tiene el enum `Result<E, T>`, definido de la siguiente manera:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Donde `T` y `E` son datos genéricos (un poco más adelante los veremos a detalle), por lo que podemos hacer que sean distintos entre sí. `T` representa el valor correcto, mientras `E` es lo que retornaremos en caso que algo falle.

Por ejemplo, podemos verificar que no se divida entre 0 de la siguiente manera:
```rust
fn main() {
    let _resultado = dividir(1, 0); // probamos con 1/0
    let resultado = dividir(1, 1);  // probamos con 1/1

    match resultado {   // verificamos el resultado
        Ok(r) => println!("El resultado es {r}."),
        Err(_) => (),
    }
}

fn dividir(a: i32, b: i32) -> Result<i32, ()> {
    if b == 0 { // verificará si el denominador es 0, si lo es retornará
        println!("Ha intentado dividir entre cero. Lo cual no es válido en nuestro sistema algebraico, y posiblemente en ningún otro. Le recomendamos no volverlo a intentar para evitar posibles consecuencias catastróficas.");
        Err(()) // retorno "vacío"
    }
    else {
        Ok(a / b)   // retorno en caso que sí sea válido
    }
}
```

Aquí la función `dividir()` verifica que no se divida entre 0. Si se trata de dividir entre 0, entonces devolverá un error. El enum `Result` nos ayuda a esto. A la hora de utilizar el resultado de la división deberemos verificar cómo quedó y "sacarlo" con un `match`. Esto ya que por defecto será un valor de tipo `Result<i32, ()>`, lo cual es útil para evitar errores, pero no es utilizable por sí solo, debemos sacarlo.

También podemos personalizarlo más creando nuestros propios tipos de errores (o utilizando los que las librerías que utilizemos han creado), por ejemplo, supongamos que no vamos a permitir que el denominador sea negativo tampoco:
```rust
fn main() {
    let resultado = dividir(1, 0); // probamos con 1/0
    verificar(resultado);   // nos dirá que tratamos de dividir entre 0

    let resultado = dividir(1, -1); // probamos con 1/-1
    verificar(resultado);   // nos dirá que tratamos de dividir entre un negativo
    
    let resultado = dividir(1, 1);  // probamos con 1/1
    verificar(resultado);   // nos dará el resultado
}

fn dividir(a: i32, b: i32) -> Result<i32, Errores> {    // retornaremos un tipo de error, si se da uno
    if b == 0 {     // verificará si el denominador es 0, si lo es retornará
        // no diremos nada aquí
        Err(Errores::Divisor0) // retorno el error
    }
    else if b < 0 {
        // retornaremos, pero no diremos nada
        Err(Errores::DivisorNegativo)  // retorna el error
    }
    else {
        Ok(a / b)   // retorno en caso que sí sea válido
    }
}

// creamos un enum con los posibles casos
enum Errores {
    Divisor0,
    DivisorNegativo
}

// haremos la verificación en una función para no tener que repetirla
fn verificar(resultado: Result<i32, Errores>) {
    match resultado {   // verificamos el resultado
        Ok(r) => println!("El resultado es {r}."),
        Err(error) => match error { // verificaremos el error
            Errores::Divisor0 => println!("Ha intentado dividir entre cero. Lo cual no es válido en nuestro sistema algebraico, y posiblemente en ningún otro. Le recomendamos no volverlo a intentar para evitar posibles consecuencias catastróficas."),
            Errores::DivisorNegativo => println!("Ha intentado dividir entre un negativo. Lo cual usualmente es válido, pero necesitabamos que no lo fuera en esta ocasión."),
        },
    }
}   
```

Alternativamente, si no manejamos los errores de una forma tan específica, sino que queremos que el programa termine al conseguir un error (por ejemplo, que el programa termine si se trató de dividir entre 0) podemos usar simplificarlo tanto con `.unwrap()` como con `expect()`. Ambos métodos retornarán el valor, si el resultado salió bien (`Ok(E)`, retornará `E`) y si sale mal llama a `panic!()`.

Con `.unwrap()`:
```rust
fn main() {
    let resultado = dividir(1, 1).unwrap();  // probamos con 1/1
    println!("El resultado es {resultado}");    // imprimirá "El resultado es 1"

    let resultado = dividir(1, 0).unwrap(); // probamos con 1/0
    // llamará a panic!()
    println!("El resultado es {resultado}");

}

fn dividir(a: i32, b: i32) -> Result<i32, ()> {
    if b == 0 { // verificará si el denominador es 0, si lo es retornará
        Err(()) // retorno "vacío"
    }
    else {
        Ok(a / b)   // retorno en caso que sí sea válido
    }
}
```

Con `.expect()`:
```rust
fn main() {
    let resultado = dividir(1, 1)   // probamos con 1/0
        .expect("Ha intentado dividir entre cero. Lo cual no es válido en nuestro sistema algebraico, y posiblemente en ningún otro. Le recomendamos no volverlo a intentar para evitar posibles consecuencias catastróficas."); 
    println!("El resultado es {resultado}");

    let resultado = dividir(1, 0)   // probamos con 1/0
        .expect("Ha intentado dividir entre cero. Lo cual no es válido en nuestro sistema algebraico, y posiblemente en ningún otro. Le recomendamos no volverlo a intentar para evitar posibles consecuencias catastróficas."); 
    // llamará también a panic!(), pero le dará un mensaje.
    println!("El resultado es {resultado}");

}

fn dividir(a: i32, b: i32) -> Result<i32, ()> {
    if b == 0 { // verificará si el denominador es 0, si lo es retornará
        Err(()) // retorno "vacío"
    }
    else {
        Ok(a / b)   // retorno en caso que sí sea válido
    }
}
```

Rust también posee un concepto muy cercano a la programación funcional para manejar errores que puedan ocurrir en distintas partes de distintas formas. Ese concepto es el de propagación de errores. Supongamos que tenemos una función que llama varias funciones que pueden fallar, la manera más sencilla de abordar esto es ir avanzando en el flujo del programa hasta que algo falle, momento en el que todo se retorna hasta el punto en que se llamó la función orinalmente, lugar donde se manejarán los errores. Por ejemplo, supongamos que deseamos crear un programa que evalúe la función $f(x) = \frac{1}{1 - \sqrt{x -1}}$, podemos programarlo de la siguiente forma tomando en cuenta su dominio $\forall x \in \mathbb{R} \;\text{s.t.}\; x\ge 1 \land x\neq 2$:
```rust
fn main() {
    let x = -1.0;

    let y = evaluar(x); // evaluará la función

    // luego verificará si salió bien.
    match y {
        Ok(v) => println!("La función evaluada en {x} es {v}."),
        Err(error) => {
            match error {
                Errores::RaizNegativa => println!("Ha intentado calcular la raíz de un número negativo, lo cual no es posible en nuestro plano actual (los reales)."),
                Errores::Denominador0 => println!("Ha intentado dividir entre 0, lo cual no es válido en este, ni en ningún otro sistema algebraico coherente.")
            }
        }
    }
}

// creamos un enum con los posibles errores
enum Errores {
    Denominador0,
    RaizNegativa,
}

// esta función evaluará la función
// 1/(1 - sqrt(x - 1))
fn evaluar(x: f64) -> Result<f64, Errores> {
    let raiz = raiz(x - 1.0);   // calculará la parte de la raíz primero
    let division = match raiz { // verificará que no haya fallado la raíz
        Ok(r) => dividir(1.0, 1.0 - r), // si todo está bien dividirá 
        Err(e) => Err(e)    // si algo salió mal, retornará el error
    };

    division    // retornará la división
    // la división puede, o no, ser exitosa
    // eso lo verifica el main
} 

// esta función evaluará la raíz
fn raiz(x: f64) -> Result<f64, Errores> {
    if x >= 0.0 {
        Ok(x.sqrt())
    }
    else {
        Err(Errores::RaizNegativa)
    }
}

// esta función divide
fn dividir(a: f64, b: f64) -> Result<f64, Errores> {
    if b == 0.0 {
        Err(Errores::Denominador0)
    }
    else {
        Ok(a/b)
    }
}
```

La ventaja de hacerlo de esta manera es la capacidad que nos otorga para reutilizar ciertas partes de nuestro código (cosa muy común en Programación Funcional) pues hay funciones (como `raiz()` o `dividir()`) que pueden ser fácilmente reutilizadas pues son bastante generales.

Como la propagación de errores es algo muy común en Rust, tenemos también una forma de simplificarlo utilizando el operador `?` que ayuda a propagar los errores de manera automática. Por ejemplo, podemos reescribir la función `evaluar()` de la siguiente manera:
```rust
fn evaluar(x: f64) -> Result<f64, Errores> {
    let raiz = raiz(x - 1.0)?;
    let division = dividir(1.0, 1.0 - raiz)?;
    Ok(division)
} 
```

El operador `?` funciona como si fuera un bazo del match, devuelve el valor o retorna el error.

# Tipos de datos genéricos

Como sabemos, Rust es estricto en los tipos, lo que significa que una función debe recibir como parámetros un tipo de dato específico y ningún otro tipo será válido. Pero hay ocasiones que esto no es muy rentable, ya que puede llegar a generar redundancia de código. Por ejemplo, imaginemos que tenemos una función que calcula el promedio de un arreglo de números:
```rust
fn main(){
    let arr = vec![14, 38, 20, 63, 38];
    let mean = promedio(&arr);
    println!("Promedio = {mean}.")
}

fn promedio_i32(list: &[i32]) -> f64 {
    let mut suma = 0;
    for e in list {
        suma += e;
    }
    suma as f64 / list.len() as f64
}
```
Pero, ¿y si los datos no son `i32`, sino `u32`? Entonces la función sería:
```rust
fn promedio_u32(list: &[u32]) -> f64 {
    let mut suma = 0;
    for e in list {
        suma += e;
    }
    suma as f64 / list.len() as f64
}
```

¿Y si son `f64`? ¿O `i64`?, etc.

Claro, esto no es buena idea, sería mejor poder ingresar muchos tipos a la vez. Después de todo, se hace lo mismo en todos los casos. En lenguajes de tipado débil, como Python, esto no es un problema. Pero en otros lenguajes, como C++ esto se soluciona por medio de datos genéricos, que representa muchos tipos a la vez. En C++ estos datos genéricos son las `template`, en Rust son los genéricos.

De la siguiente manera:
```rust
fn promedio<T>(list: &[T]) -> f64 {
```

De momento esto no va a compilar, debido a que debemos implementar traits, que veremos más adelante. Retomaremos este ejemplo cuando veamos traits.

Por otro lado, podemos también definir estructuras con datos genéricos, por ejemplo, una que tome coordenadas:
```rust
fn main() {
    let punto_i32 = Point{x: 1, y: 2, z: 3};
    let punto_u32: Point<u32, u32, u32> = Point{x: 1, y: 2, z: 3};
    let punto_f64 = Point{x: 1.1, y: 2.2, z: 3.3};
}

// los datos genéricos serán T, U y V
struct Point<T, U, V> {
    x: T,
    y: U,
    z: V,
}
```

Al igual que podemos tener enums de datos genéricos, y ya hemos visto y usado algunos. Por ejemplo `Option<T>` y `Result<T, E>`, ambos son enums con datos genéricos:
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Y de igual forma, si necesitamos crear métodos para estructuras o enums, necesitamos utilizar los datos genéricos con `impl`:
```rust
fn main() {
    let punto_i32 = Point{x: 1, y: 2, z: 3};
    let punto_u32: Point<u32, u32, u32> = Point{x: 1, y: 2, z: 3};
    
    let punto_f64 = Point{x: 1.1, y: 2.2, z: 3.3};
    let (x, y, z) = punto_f64.extraer();
    println!("El punto es ({x}, {y}, {z})");
}

struct Point<T, U, V> {
    x: T,
    y: U,
    z: V,
}

impl <T, U, V>  Point<T, U, V> {
    fn extraer(&self) -> (&T, &U, &V){
        (&self.x, &self.y, &self.z)
    }
}
```

Y para aclarar, el uso de genéricos no afecta el rendimiento de nuestro programa ni su seguridad. El compilador se encargará de reemplazar nuestros genéricos por códio real para que se ejecute sin perder rendimiento. Al igual que verifica que no se pueda hacer nada no válido con los genéricos mediante traits.

# Traits

Los traits (rasgo) define las funcionalidades de un tipo de datos y como puede compartirlas con otros tipos.

Estos pueden definir características, y cómo serán, para uno o varios tipos. Por ejemplo, imaginemos que tenemos dos estructuras: `Vegetal` y `Fruta`, y queremos que ambas sean "comestibles". Podemos crearles un trait para eso:
```rust
// definimos la estuctura Vegetal
struct Vegetal {
    nombre: String,
}

// definimos la estructura Fruta
struct Fruta {
    nombre: String,
}

// definimos el trait Comestible
trait Comestible {
    fn comer(&self);    // también podríamos hacer que retorne algo, igual que con las funciones
}
```

Pero, hasta ahora solo lo estamos definiendo. Debemos implementarlo. Podemos lograrlo con `impl` de la siguiente manera:
```rust
impl Comestible for Vegetal {
    fn comer(&self) {
        println!("Has comido un vegetal llamado {}", self.nombre);
    }
}

impl Comestible for Fruta {
    fn comer(&self) {
        println!("Has comido una fruta llamada {}", self.nombre);
    }
}
```

Y ahora podemos usar `.comer()` en ambas:
```rust
fn main(){
    let zanahoria = Vegetal {nombre: "Zanahoria".to_string()};
    zanahoria.comer();

    let manzana = Fruta {nombre: "Manzana".to_string()};
    manzana.comer();
}
/*
    Has comido un vegetal llamado Zanahoria
    Has comido una fruta llamada Manzana
*/
```

De igual forma, podemos hacer que los traits tengan funciones por defecto. Por ejemplo, podemos hacer que siempre diga `"Has comido"`:
```rust
// definimos el trait
trait Comestible {
    fn comer(&self) {   // definimos la función
        println!("Has comido");
    }
}

impl Comestible for Vegetal {}  // implementamos el trait

impl Comestible for Fruta {}    // implementamos el trait
```

Al igual que podemos tener varias funciones, hacer que se llamen entre sí incluso si no se sabe qué hará una de ellas:
```rust
// definimos el trait
trait Comestible {
    fn comer(&self) {   // definimos función comer
        println!("Has comido {}", self.nombre());   // utilizamos el nombre que obtendremos, de alguna forma
    }

    // creamos la función para obtener el nombre
    fn nombre(&self) -> String;
}

impl Comestible for Vegetal {
    fn nombre(&self) -> String {    // definimos cómo obtenemos el nombre
        self.nombre.clone()
    }
}

impl Comestible for Fruta {
    fn nombre(&self) -> String {    // definimos cómo obtenemos el nombre
        self.nombre.clone()
    }
}
```

Gracias a los traits podemos definir funciones que puedan recibir parámetros de varios tipos siempre que tengan implementado dicho trait. Por ejemplo, podemos definir una función que utilize el trait `Comestible` de nuestro ejemplo:
```rust
fn hacer_jugo(item: &impl Comestible) {
    println!("Has hecho jugo de {}", item.nombre());
    item.comer();
}
```

A esta forma de escribirlo se le llama sintaxis `impl Trait`.

Y al llamarla:
```rust
fn main(){
    let zanahoria = Vegetal {nombre: "Zanahoria".to_string()};
    let manzana = Fruta {nombre: "Manzana".to_string()};

    hacer_jugo(&zanahoria);
    hacer_jugo(&manzana);
}
/*
    Has hecho jugo de Zanahoria
    Has comido Zanahoria
    Has hecho jugo de Manzana
    Has comido Manzana
*/
```

Esto funciona gracias a que, al "pedir" como requisito para usarse en la función, el dato que entre tendrá tanto el método `.nombre()` como el `.comer()`. Por lo que podemos usar ambos dentro de la función.

Esto también lo podemos escribir de manera más precisa usando la sintaxis _trait bound_:
```rust
fn hacer_jugo<T : Comestible> (item: &T) {
    println!("Has hecho jugo de {}", item.nombre());
    item.comer();
}
```
Hará lo mismo.

De igual forma podemos tener multiples traits como "requisitos" en una función. Por ejemplo, supongamos que queremos crear una función `funcion()` que necesite los traits de `Rasgo1` y `Rasgo2`. Entonces podemos tenerlo de las siguientes maneras:

Sintaxis `impl Trait`:
```rust
fn funcion(item: &(impl Rasgo1 + Rasgo2)) -> () {
```

Sintaxis _trait bound_:
```rust
fn funcion<T: Rasgo1 + Rasgo2>(item: &T) -> () {
```

Alternativamente, también podemos usar un bloque `where` para escribir esto mismo, aunque es más usado cuando tenemos multiples datos genéricos. Por ejemplo, supongamos que tenemos una función llamada `funcion()` que recibe como parámetros dos datos genéricos `T` y `U`, de los cuales `T` debe tener los traits `Rasgo1` y `Rasgo2`, y `U` debe tener `Rasgo3` y `Rasgo4`. Podemos escribirlo como:
```rust
fn funcion<T: Rasgo1 + Rasgo2, U: Rasgo3 + Rasgo4>(t: &T, u: &U) -> () {
```
O usando `where`, que nos quedaría:
```rust
fn function<T, U>(t: &T, u: &U) -> ()
where
    T: Rasgo1 + Rasgo2,
    U: Rasgo3 + Rasgo4,
{
```

Esto no solo es válido para funciones, sino también podemos usarlo con `impl`, permitiendo que ciertos métodos solo se implementen si se tienen ciertos traits (por ejemplo, cuando se utilizan datos genéricos en una estructura) o con los mismos traits (hacer que una estructura con ciertos traits tenga tambien otros traits).

Por último, retomando lo que habíamos dicho que retomaríamos de la función `promedio()`, quedaría así:
```rust
use std::ops::AddAssign;    // trait de uso de +=
use std::convert::TryInto;  // trait de conversiones

fn main(){
    let arr = vec![14, 38, 20, 63, 38];
    let mean = promedio(&arr);
    println!("Promedio = {mean}.")
}

fn promedio<T> (list: &[T]) -> f64 
where 
    T: AddAssign + Copy + Into<f64>,
{
    let mut suma = 0.0;
    for e in list {
        suma += (*e).into();
    }
    suma / list.len() as f64
}
```

# Lifetime
Cap. 10.3

# Pruebas (`test`)
Cap. 11