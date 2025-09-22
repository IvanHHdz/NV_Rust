# Unsafe

Hasta el momento, hemos trabajado con Rust utilizando su seguridad. Como dijimos al principio, el compilador de Rust nos dice que no compilará para protegernos. ¿Protegernos de qué? De posibles errores, de posibles fallos que él puede detectar.

El compilador, junto con todas las demás reglas que posee Rust nos garantizan seguridad, y para la mayor parte de lo que necesitamos hacer, eso es perfecto. Sin embargo, el compilador de Rust es muy estricto; conservador, por así decirlo. Por lo que en ocasiones no nos dejará compilar código que claramente funciona sin errores. Esto ocurre porque el compilador no puede saberlo todo de nuestro programa, por lo que trabaja en base a lo que sabe. Y en ocasiones, según lo que sabe, nuestro código no es para nada seguro, por eso lo rechaza. Pero claro, nosotros podemos saber más que el compilador.

Por esta razón, existe `unsafe rust`. Unsafe es una forma en la que nosotros tomamos el control, el compilador confiará que nosotros verificaremos que, por ejemplo, que todo es válido. De esa forma, somos nosotros los que tenemos la responsabilidad, porque le decimos al compilador: "tranquilo, sé lo que hago" y el confía.

Aun así, el compilador no nos dejará completamente solos, sino que verificará cosas, como el _borrowing_.

## Raw Pointer

Como sabemos, según las reglas del _ownership_, no podemos tener multiples referencias mutables, ni tener referencias inmutables si existe una mutable. Es por esta razón por la que algo como lo siguiente no compila:
```rust
fn main() {
    let mut x = 5;

    let x1 = &x;        // creamos una referencia inmutable
    let x2 = &mut x;    // creamos una referencia mutable, por lo que x1 deja de ser válida

    *x2 += 1;           // usamos la referencia mutable

    println!("x1: {}, x2: {}", x1, x2); // volvemos a usar la referencia inmutable (esta no es valida) y la referencia mutable
    // no compilará porque tratamos de usar x1, la cual es inválida.
}
```

Esto no es válido. Sin embargo, podemos hacerlo haciendo uso de los _raw pointer_, los cuales son variaciones de los punteros para unsafe. Estos no están obligados a seguir las reglas del _ownership_. De igual forma que para los punteros normales, estos tienen versiones mutables e inmutables: `&raw mut` y `&raw const` respectivamente. Es posible crearlos fuera de unsafe, pero no podemos desreferenciarlos fuera, por lo que son prácticamente inútiles sin unsafe. Veamos un ejemplo:
```rust
fn main() {
    let mut x = 5;

    // creamos los punteros raw
    let x1 = &raw const x;  // inmutable
    let x2 = &raw mut x;    // mutable

    unsafe {        // abrimos el bloque unsafe
        *x2 += 1;   // modificamos el valor del puntero mutable 
        // como es un puntero raw, x1 sigue siendo válido

        println!("x1: {}, x2: {}", *x1, *x2);   // imprimimos el puntero inmutable y el mutable
        // renemos que desreferenciarlos porque son tipo *const i32 y *mut i32
        // y ninguno de esos tipos implementa Display
    }   // cerramos el bloque unsafe
    
}
```

Esto compila sin problemas.

Sin embargo, es importante tener en cuenta que debemos ser cuidadosos con estos punteros. Después de todo, ignoran ciertas partes de rust que están por razones importantes.

## Funciones o métodos unsafe

Una función unsafe es aquella que el compilador no garantiza que sea segura. Se declaran con un `unsafe` antes del resto de la definición de la función:
```rust
unsafe fn saludo() {
    println!("Este es un saludo no seguro!");
}
```

Esta función solo puede ser llamada dentro de un bloque `unsafe`. 

A continuación, un ejemplo útil de una función unsafe. Consideremos lo siguiente: queremos una función que tome una slice y la "corte" en cierto punto.
```rust
fn main() {
    // creamos la lista
    let mut l = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // la imprimimos para verla
    println!("{:?}", l);

    // llamamos la función cortar()
    let (l1, l2) = cortar(&mut l, 5);

    // imprimimos las dos mitades
    println!("{:?} / {:?}", l1, l2);
}

fn cortar(lista: &mut [i32], corte: usize) -> (&mut [i32], &mut [i32]) {
    (&mut lista[..corte], &mut lista[corte..])
}
```
Por cómo está hecha la función, sabemos que no estamos haciendo una referencia doble a ningún elemento. pues cada elemento está o en una mitad, o en la otra: no en ambas.

Sin embargo, el compilador no puede saber eso, así que no permitirá que este código compile. Es por eso que debemos hacerlo de forma especial con unsafe, junto a algunas funciones especiales de `slice`:
```rust
use core::slice;

fn main() {
    // creamos la lista
    let mut l = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // la imprimimos
    println!("{:?}", l);

    // cortamos la lista, necesitamos usar unsafe aquí
    let (l1, l2) = unsafe { cortar(&mut l, 5)};

    // imprimimos para verificar
    println!("{:?} / {:?}", l1, l2);
}

// declaración de la función unsafe
unsafe fn cortar(lista: &mut [i32], corte: usize) -> (&mut [i32], &mut [i32]) {
    let l = lista.as_mut_ptr();     // la convertimos en un puntero raw
    unsafe {
        // la "cortamos" haciendo uso de esta función
        (
            slice::from_raw_parts_mut(l, corte),
            slice::from_raw_parts_mut(l.add(corte), lista.len() - corte)
        )
    }
}
```

## Código Externo

Rust provee el llamado _Foreign Function Interface (FFI)_, que nos permite llamar funciones de otros lenguajes. Por ejemplo, podemos llamar la función `abs()` de la librería estándar de C. 

Sin embargo, es importante recalcar que, por razones un poco obvias, el compilador de Rust no puede saber si esas funciones son seguras, por lo que se asume que no. Así que son tomadas como _unsafe functions_, que deben ser llamadas desde un bloque `unsafe`.

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Valor absoluto de -3 según C: {}", abs(-3));
    }
}
```

El `extern` nos ayuda a definir las funciones que llamaremos desde otro lenguaje. el `"C"` se refiere a cual _application binary interface (ABI)_ utiliza dicho lenguaje, esto indica cómo llamar la función a nivel ensamblador, siendo `"C"` el más común y que hace referencia a como lo hace C.

Como sabemos que esta función es segura, podemos usar `safe` para "prometerle" al compilador, por así decirlo, que esa función es segura y que no hay problemas. 

## Variables globales

Hasta el momento, hemos ignorado un concepto que puede resultar importante y de interés: las variables globales. Las variables globales son algo especiales en Rust, pues existen de dos tipos: las `const` (que las vimos al inicio) y las `static`. Las `const` con, como indica su nombre, constantes; no solo por defecto, sino siempre. Por otro lado, las `static` sí pueden llegar a ser mutables, con el detalle que modificarla y acceder a ellas es unsafe:
```rust
static mut GLOBAL_VAR: i32 = 0;

fn main() {
    unsafe {
        GLOBAL_VAR += 1;
        println!("GLOBAL_VAR: {}", *(&raw const GLOBAL_VAR));
    }
    println!("{}", unsafe { *(&raw const GLOBAL_VAR) });
}
```

Esto funciona así por la posibilidad que existe que tratemos de hacer lo mismo pero en un entorno con multiples hilos, lo cual puede llegar a ser problematico y el compilador no sabría si es, o no, seguro. Razón por la cual asume que es inseguro y nos deja a nosotros tomar la responsabilidad para garantizar la seguridad.

En fin, estos solo son algunos de los usos y formas de usar unsafe. Mucho más adelante aprenderemos más a detalle sobre esto, gracias al _Rustonomicon_.

# Macros

A lo largo de todo el curso, hemos utilizado macros: `println!()`, `dbg!()`, `format!()`, etc. Sin embargo, no hemos explicado a fondo las macros aún; no hemos visto a detalle qué son ni cómo funcionan.

Las macros son una forma de _metaprogramación_ que nos permite programar código que se encargará de crear código por nosotros. Un ejemplo sencillo de esto es `format!()`, la cual se encarga se escribir correctamente la concatenación de cadenas de texto.

Hasta cierto punto, podría pensarse que son como funciones, y si bien tienen similitudes, las macros son mucho más complejas que las funciones. Después de todo, estamos escribiendo código que escribe código.

## Macros declarativas

Se puede pensar en ellas como algo paracido a los `match`. Dado un pattern, se ejecuta un cierto código; aunque en este caso no se ejecuta, sino que se reemplaza. Veamoslo con un ejemplo, una macro que realiza una suma:
```rust
fn main(){
    let resultado = suma!(1, 2, 3, 4, 5);
    println!("El resultado es: {}", resultado);
}

#[macro_export]     // necesario para poder llamar a la macro
macro_rules! suma { // usamos macro_rules! para declararla
    ( $( $x:expr ),* ) => {     // indicamos que se tomara un x, que es una expresión
                                // la ',' indica otro parámetro
                                // y la * indica que se puede repetir 0 o más veces lo anterior
        {
            let mut sum = 0;
            $(              // de esta forma abrimos algo similar a un bucle, que tomará todas las x que ingresemos
                sum += $x;
            )*
            sum     
        }
    };  // aquí termina la declaración de la macro, o el código con el que será reemplazada
}
```

Como tal, el compilador reemplazará cada llamada por el código correspondiente (lo cual es distinto a llamar una función). En este caso, después del reemplazo, quedaría de la siguiente manera:
```rust
fn main(){
    let resultado = {
            let mut sum = 0;
            sum += 1;
            sum += 2;
            sum += 3;
            sum += 4;
            sum += 5;
            sum     
        };
    println!("El resultado es: {}", resultado);
}
```

Aquí otro ejemplo, más completo:
```rust
fn main(){
    let persona = crear_persona!("Juan", 30, "Calle Falsa 123", "555-1234", "juan@example.com");
    println!("{:?}", persona);

    let persona = crear_persona!("Pablo", 41);
    println!("{:?}", persona);
}

#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: u8,
    direccion: String,
    telefono: String,
    email: String,
}

#[macro_export]
macro_rules! crear_persona {
    ($nombre:expr, $edad:expr, $direccion:expr, $telefono:expr, $email:expr) => {
        Persona {
            nombre: $nombre.to_string(),
            edad: $edad,
            direccion: $direccion.to_string(),
            telefono: $telefono.to_string(),
            email: $email.to_string(),
        }
    };
    ($nombre:expr, $edad:expr) => {
        Persona {
            nombre: $nombre.to_string(),
            edad: $edad,
            direccion: "N/A".to_string(),
            telefono: "N/A".to_string(),
            email: "N/A".to_string(),
        }
    };
}
```