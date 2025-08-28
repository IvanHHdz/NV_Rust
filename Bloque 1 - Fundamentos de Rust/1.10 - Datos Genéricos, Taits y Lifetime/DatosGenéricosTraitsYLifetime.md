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

Los traits (rasgo) define las funcionalidades de un tipo de datos y como puede compartirlas con otros tipos. Estos pueden definir características, y cómo serán, para uno o varios tipos. Por ejemplo, imaginemos que tenemos dos estructuras: `Vegetal` y `Fruta`, y queremos que ambas sean "comestibles". Podemos crearles un trait para eso:
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

A esta forma de escribirlo se le llama sintaxis `impl Trait`. Y al llamarla:
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
Lifetime (tiempo de vida) de una variable se refiere al la sección de código en el que dicha variable es válida. Es algo que ya hemos estado usando, aunque no de manera explícita, ya que al igual que con los tipos, Rust puede inferir el lifetime de las variables. El detalle está en que, al igual que con los tipos, existen situaciones en las que el compilador no podrá inferir correctamente, por lo que deberemos de declararlo explícitamente.

## _Borrow Checker_
Vamos a comenzar explicando que es el _borrow checker_. Este es una herramienta que posee el compilador para verificar que todas las referencias que se realizen dentro de un programa sean válidas y no llegen a apuntar a valores indefinidos (pues Rust no tiene valores nulos). Para entender mejor cómo funciona, veamos el siguiente ejemplo:
```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```

En este caso, el lifetime de `r` es `'a` y el de `x` es `'b`. Podemos observar que hay un espacio en el que solo está `r`, sin valor. Luego aparece `x`, y `r` hace referencia a `x`. Por lo que `r` tiene valor, pero solo mientras esté en el lifetime `'b`. Al salir de ese lifetime, `r` no tiene valor pues no puede apuntar a `x` por que ya no existe. Así que no puede usarse `println!("r: {r}");` por que `r` no posee valor alguno. Razón por la cual el compilador nos impedirá compilar esto.

Esto se soluciona si extendemos el lifetime de `x` hasta donde `r` lo necesita. Por ejemplo:
```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```

## _Function Signatures_

Con lo anterior claro, pongamos el siguiente ejemplo. Supongamos que compara dos Strings y retorna la que va primero alfabéticamente. Esto es sincillo de lograr, por ejemplo:
```rust
fn main() {
    // creamos las Strings
    let s1 = String::from("Impacto");
    let s2 = String::from("Elemento");

    // creamos una slice que haga referencia a la primera
    let primero = if s1 > s2 {
            &s2
        }
        else {
            &s1
        };
    // imprimimos
    println!("La primera entre \"{s1}\" y \"{s2}\" es: \"{primero}\"");
    /*
        La primera entre "Impacto" y "Elemento" es: "Elemento"
    */
}
```

Pero, habíamos dicho que queríamos que fuera una función, así que vamos a programarla:
```rust
fn primero(s1: &str, s2: &str) -> &str {
    if s1 > s2 {
        &s2
    }
    else {
        &s1
    }
}
```
Parece correcto, es lo mismo que teníamos. Así que debería funcionar. ¿No? No, no compilará. Esto debido a que la referencia que retornará la función tiene lifetime indefinido. Rust no sabe qué lifetime tendrá, dónde termina ni cuando deja de ser válido. Y el mayor problema es que, teóricamente, nosotros tampoco sabemos.

Para solucionar esto, Rust introduce las anotaciones de lifetime, las cuales son lifetime genéricas. Lo que haremos es indicar que el valor de retorno poseerá un lifetime que termina donde termina el primer lifetime de los parámetros. Es decir, terminará donde termine el primero. Esto lo hacemos de la siguiente manera:
```rust
fn primero<'a> (s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        &s2
    }
    else {
        &s1
    }
}
```

De esta forma estamos diciendo que el lifetime de retorno será la intersección de los lifetimes de ambos parámetros. El ejemplo completo nos quedaría de la siguiente forma:
```rust
fn main() {
    // creamos las Strings
    let s1 = String::from("Impacto");
    let s2 = String::from("Elemento");

    // creamos llamamos la función
    let primero = primero(&s1, &s2);

    // imprimimos
    println!("La primera entre \"{s1}\" y \"{s2}\" es: \"{primero}\"");
    /*
        La primera entre "Impacto" y "Elemento" es: "Elemento"
    */
}   // aquí termina el lifetime de s1 y s2. por lo que también termina el de 'a

// declaramos la función
fn primer<'a> (s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        &s2
    }
    else {
        &s1
    }
}
```

Hay que tener en consideración lo que dijimos antes, que solo es la **intersección**. Veamoslo mejor en el siguiente ejemplo:
```rust
fn main() {
    // creamos la String
    let s1 = String::from("Impacto");
    let primero: &str;

    {
        // creamos la String
        let s2 = String::from("Elemento");

        // creamos llamamos la función
        let primero = primer(&s1, &s2);
    }   // aquí termina el lifetime de s2, por lo que el de 'a también termina aunque el de s1 siga
    // imprimimos
    println!("La primera es: \"{primero}\"");
}   // aquí termina el lifetime de s1 
```

Este código no compilará, ya que se estará tratando de llamar un valor cuando su lifetime ya terminó. Lo cual el _borrow checker_ detectará.

Esto mismo, al igual que con los datos genéricos, funciona igual con las estructuras, enums, métodos, etc., siguiendo siempre el mismo patrón. Por ejemplo, para estructuras:
```rust
struct Estructura<'a> {
    cadena: &'a str,
}
```