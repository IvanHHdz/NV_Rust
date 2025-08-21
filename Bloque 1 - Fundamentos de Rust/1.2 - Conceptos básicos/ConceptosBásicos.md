# Variables y mutabilidad

En Rust, para declarar una variable utilizamos la siguiente sintaxis:
```rust
let numero = 5;
```

Muy similar a como se declaran en otros lenguajes (como JavaScript). Es importante destacar que aquí todas las variables son inmutables (es decir, no pueden cambiar) por defecto.

Esto quiere decir que si tengo:
```rust
let a = 1;
a = 3;
```
Obtendré un error al intentar compilarlo (`cannot assign twice to immutable variable a`), pues `a` es inmutable y por lo tanto no es posible cambiar su valor. A menos, claro, que la reescribamos:
```rust
let a = 1;
let a = 3;
```

Esta vez sí compilará. Pero, hay ocasiones en que preferimos editar una variable en lugar de sobreescribirla. Para hacer esto debemos volverla mutable de manera explícita, por medio de la palabra reservada `mut`:
```rust
let mut a = 1;
a = 3;
```

Y así, podemos tener tanto variables mutables como inmutables. Rust utiliza las inmutables por defecto con la intensión de evitar cambios accidentales o inesperados. Así que se recomienda dejarlas inmutables a menos que realmente se requiera que sea mutable.

```rust
let a = 0;      // inmutable
let mut b = 1;  // mutable
```

> Nota, los comentarios en Rust se colocan igual que en C++: con `//` o con `/* */`.

# Tipos de datos

Rust, al igual que lenguajes como C++, posee un tipado estático y fuerte. Lo que quiere decir que es necesario conocer el tipo de todos los datos que se van a utilizar en tiempo de compilación, y que los cambios de tipo se deben hacer por medio de conversiones explícitas (veremos esto más adelante).

Aún así, Rust es capaz de inferir el tipo de dato en la mayoría de los casos. Aún así, habrán ocasiones en las que deberemos especificarle el tipo.

Por defecto, Rust cuenta con datos:
- Numéricos con signo (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`; un tipo por cada tamaño)
- Numéricos sin signo (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`)
- Decimales o de punto flotante (`f32`, `f64`)
- Booleanos (`bool`)
- Carácteres (`char`)

Es importante mencionar que los numéricos pueden ser escritos de varias formas:

| Tipo      | Ejemplo       |
|-----------|---------------|
| Decimal   | `2_005`       |
| Hex       | `0xff`        |
| Octal     | `0o77`        |
| Binario   | `0b1111_0000` |
| Byte      | `b'A'`        |

Notemos que el 2005 tiene un `_`, es simplemente para facilitar su lectura. Pues es más sencillo entender el número si está escrito como 2,005 que si es 2005; especialmente para número más grandes. Rus simplemente ignora los `_`.

Para especificar el tipo de dato, como mencionabamos anteriormente, debemos colocar `:` y el tipo después del nombre. Por ejemplo:
```rust
let num : i32 = -1_200;
let num2 : u32 = 300;
let dec : f64 = 3.14;
let boolean : bool = true;
let caracter : char = 'A';
```

# Operaciones

Y de igual forma que en otros lenguajes, tenemos una serie de operaciones que se pueden realizar dependiendo del tipo. Pero, diferencia de otros lenguajes como Python, Rust **no hará conversión implícita** durante ninguna operación. Por lo que hay que ser cuidadosos en esa parte. Rust no permite mezclar tipos.

Los datos numéricos y decimales admiten operaciones aritmeticas:
```rust
let a = 10;  
let b = 3;
// puesto que no se especificó el tipo, se infiere que ambos son i32

let suma       = a + b; // 13
let resta      = a - b; // 7
let producto   = a * b; // 30
let division   = a / b; // 3 (entero)
let resto      = a % b; // 1
```

De manera similar a C++, si deseamos una división decimal, los datos deben ser decimales:
```rust
let x = 5 / 2;     // 2 (i32)
let y = 5.0 / 2.0; // 2.5 (f64 por defecto)
```

Con la diferencia que no podemos mezclar tipos:
```rust
// let z = 5 / 2.0; //  error, estamos dividiendo un i32 entre un f64
let z = 5 as f64 / 2.0; // 2.5
```

Al igual que C++ y Python, podemos:
```rust
let mut n = 5;
n += 2; // 7
n *= 3; // 21
n -= 1; // 20
n /= 4; // 5
```

Siempre que la variable sea mutable, claro.

También tenemos operaciones lógicas, iguales a las de C++:
```rust
let a = true && false; // false
let o = true || false; // true
let n = !true;         // false
```
Y operaciones de comparación:
```rust
let x = 5;
let y = 3;

println!("{}", x > y);  // true
println!("{}", x <= y); // false
```

También tenemos operaciones binarias (solo para enteros):
```rust
let a = 0b1010; // 10
let b = 0b1100; // 12

let and  = a & b; // 0b1000 (8)
let or   = a | b; // 0b1110 (14)
let xor  = a ^ b; // 0b0110 (6)
let not  = !a;    // complemento a dos
let despl_izq = a << 1; // 0b10100 (20)
let despl_der = b >> 2; // 0b0011  (3)
```


# Input/Output

Como vimos anteriormente, la forma con la que se imprime información en pantalla es por medio de la macro `println!()`, al mandar una cadena de texto como parámetro. 

Es posible, al igual que en otros lenguajes de programación, añadir partes a estas cadenas de texto directamente. Por ejemplo, supongamos que queremos saludar directamente al usuario por su nombre, que conocemos y guardamos en la variable `nombre`:
```rust
fn main(){
    let nombre = "Ferris";
    println!("Hola {}!", nombre);
}
```
```
Hola Ferris!
```

Que también podemos colocar dentro del texto, de manera similar a las cadenas de texto formateadas (`f"{}"`) de Python:
```rust
fn main(){
    let nombre = "Ferris";
    println!("Hola {nombre}!");
}
```
Obteniendo el mismo resultado.

En cuanto a recibir input por parte del usuario. Necesitaremos primero hacer una importación de la librería estándar de Rust. 

Rust, al igual que otros lenguajes como C++, posee una librería estandar (`std`) la cual contiene muchas cosas que nos pueden resultar útiles. Para importar específicamente la parte que nos interesa ahora (`io`, para input/output) la importaremos de la siguiente manera:
```rust
use std::io;
```
Similar a como se hace en C++.

Ahora bien, para leer lo que ingrese el usuario en una variable de tipo `String`, y luego usamos lo siguiente:
```rust
 // creamos una cadena vacía en la que guardaremos lo que ingrese el usuario
let mut texto = String::new();
io::stdin() 
    .read_line(&mut texto)  
    .expect("Error");
```

Explicando por partes:
```rust
io::stdin()
```
Del módulo `io`, usamos la función `stdin()`, la cual devuelve un valor de tipo `std::io::Stdin`.

Podiamos habernos ahorrado esto al importar directamente la función que necesitabamos, es decir, con:
```rust
use std::io::stdin;
```
Con lo que solo usaríamos `stdin()` directamente.

Al igual que podiamos ahorarnos la importación del principio si usamos aquí todo el comando completo:
```rust
std::io::stdin()
```
Así no es necesario el `use` del principio.
```rust
.read_line(&mut texto)  
```
Al valor retornado, le aplica el método `.read_line()`, que toma como parámetro el lugar donde se guardará el dato ingresado por el usuario.
```rust
.expect("Error");
```
Y si, por alguna razón, algo llegara a fallar, manejamos ese error por medio del método `expect()`, que recibe como parámetro el mensaje que mostrará en caso de error.

Y así, al ejecutarse esa parte, tendremos el valor que el usuario haya ingresado dentro de la variable `texto`.

# Importaciones

Ahora hablaremos un poco de cómo se realizan importaciones de dependencias externas. Para esta parte `cargo` nos ayuda mucho.

En Rust, se refieren a las librerías como `librery crate` y los programas como `binary crate`, entendiendo `crate` como un conjunto de archivos de Rust. 

Si creamos un proyecto:
```shell
cargo new ejemplo
```

Encontraremos un archivo nuevo llamado `Cargo.toml` en `ejemplo > Cargo.toml`. En la parte de abajo, encontraremos lo siguiente:
```toml
[dependencies]
```

Seguido de lo cual podemos colocar las `library crate` que necesitemos de forma similar a si estuvieramos escribiendo un `requirements.txt` para Python. Por ejemplo, si necesitamos utilizar números aleatorios:
```toml
[dependencies]
rand = "0.9.2"
```
Y esto instalará las dependencias necesarias. Y, por ejemplo, para generar un número aleatorio, bastará con usar:
```rust
// importamos la librería rand
use rand::Rng

fn main(){
    // creamos una variable con un valor aleatorio
    let secret_number = rand::rng().random_range(1..=100);
    // y lo imprimimos
    println!("{}", secret_number);
}
```

Esto nos imprimirá un número aleatorio entre 1 y 100.

La primera vez que lo ejecutemos con `cargo run` tardará un rato pues instalará `rand`. Después tardará lo usual.

# `const`

Al igual que en otros lenguajes, Rust posee constantes, que son datos que no cambiarán en el programa. Al igual que las variables son inmutables por defecto, con la diferencia que las constantes no se pueden volver mutables. Se declaran usando `const` en lugar de `let` y se debe de especificar su tipo al declararse.

Pueden declararse en cualquier bloque, para que poseen cualquier alcance, aunque lo usual es declararlas con alcance global. 

El valor de estas debe ser conocido en tiempo de compilación, lo que quiere decir que no puede ser el resultado de una expresión que requiera algún dato que se conozca solo en ejecución. 

Ejemplo de constante:

```rust
const PI : f64 = 3.1415926;
```