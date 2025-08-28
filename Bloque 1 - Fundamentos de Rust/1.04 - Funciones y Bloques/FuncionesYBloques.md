# Funciones

Ya desde el inicio habíamos mencionado que en Rust hay funciones y hemos estado viendo una de estas: `main()`.

En Rust la declaración de funciones es similar a otros lenguajes como Python. Utilizamos la palabra reservada `fn` seguido del nombre, paréntesis en donde irán los parámetros, el retorno y su respectivo bloque de código entre `{}`. Y llamarlas es igual que en otros lenguajes, colocamos el nombre de la función junto a `()` con sus parámetros.

Un ejemplo sencillo puede ser:
```rust
// declaramos la función saludo
fn saludo(){
    println!("Hola a todos!");
}

// declaramos la función main
fn main(){
    // llamamos a la función saludo()
    saludo();
}
```

> Nota, en Rust se suele usar la misma convención que utiliza Python para nombrar funciones: _Snake Case_.

# Parámetros y retorno
De igual forma que en otros lenguajes, podemos pasar valores a las funciones por medio de parámetros. Para lo cual simplemente debemos definirlos en la declaración de la función, especificando el tipo:
```rust
fn saludo(x: i32){
    for _ in 1..=x {
        println!("Hola!");
    }
}

fn main(){
    saludo(3);
    // imprimirá 3 veces "Hola!"
}
```

Y si necesitamos multiples parámetros, los separamos por comas `,`:
```rust
// más adelante explicaremos ese &str
fn saludo(nombre: &str, x: i32){
    for _ in 1..=x {
        println!("Hola {nombre}!");
    }
}

fn main(){
    saludo("Ferris", 5);
    // saludará 5 veces a Ferris
}
```

Como tal, las funciones en Rust están compuestas por sentencias y, opcionalmente, una expresión al funal. Como mencionamos anteriormente, Rust hace una distinción que es importante tener en cuenta:
- Las sentencias son instrucciones que realizan una acción y no retornan un valor.
- Las expresiones sí returnan un valor al evaluar algo.

Como dijimos, las funciones pueden retornar valores, y esto lo hacen cuando la última línea del bloque de la función es una expresión. Al igual que en lenguajes como C++, debemos especificar el tipo de dato que a función va a retornar. Esto lo hacemos haciendo uso de `->` seguido del tipo, de la siguiente forma:
```rust
fn main(){
    let r = suma(2, 2);
    println!("{r}")
}

fn suma(a: i32, b: i32) -> i32 {
    // es importante recordar no colocar un ; al final
    // pues eso convertiría nuestra expresión en una sentencia
    a + b
}
```

# Bloques y alcance

Un bloque en Rust, como mencionábamos anteriormente, consiste en una secuencia de expresiones y/o sentencias agrupadas entre llaves `{}`. Se considera que todos los bloques tienen un valor y un tipo, determinados por su última línea.

Como mencionábamos antes, las funciones se componen de un bloque. El valor de retorno de la función será el mismo que el valor del bloque. En caso que la última línea no sea una expresión y por lo tanto no retorne un valor, se considerará que retorna una tupla vacía: `()`.

De igual forma, así como en otros lenguajes, lo que pasa dentro de un bloque se queda en ese bloque. Esto aplica para todos los bloques dentro de Rust. ¿Qué quiere decir esto? Que las variables que se creen dentro de un bloque **solo existen dentro de dicho bloque**. Pero no solo aplica para la creación de variables, toda clase de reescritura de variables. A esto se le conoce como _shadowing_.

Es decir, si reescribimos una variable dentro de un bloque, esta solo se verá afectada dentro de dicho bloque. Esto, claro, no incluye modificarla. Es decir, si **modificamos** una variable, sí se ve afectada; en cambio, si la **reescribimos** no se verá afectada fuera del bloque. Lo que puede resultar útil en ocasiones.

Aquí un ejemplo, para entenderlo mejor:
```rust
fn main(){
    let x = 3;              // la dejamos inmutable, pues no la modificaremos
    println!("{x}");        // imprime 3
    {
        let x = 23;         // reescribimos el valor
        println!("{x}");    // imprime 23
    }                       // termina el bloque y x retoma su valor antes del bloque
    println!("{x}");        // imprime 3
}
```

Y ahora, modificando:
```rust
fn main(){
    let mut x = 3;          // la volvemos mutable pues la modificaremos
    println!("{x}");        // imprime 3
    {
        x = 23;             // modificamos el valor
        println!("{x}");    // imprime 23
    }                       // como la modificamos, el cambio se mantiene
    println!("{x}");        // imprime 23
}
```

Esto funciona precisamente por el alcance que tiene cada variable. En el ejemplo de reescritura la variable `x` solo existe dentro del bloque en donde se le asigna el valor `23`, fuera de dicho bloque existe con el valor de `3`. Lo mismo aplicaría si no es declarada afuera:
```rust
fn main(){
    {
        let x = 23;         // creamos la variable con su valor
        println!("{x}");    // imprime 23
    }                       // termina el bloque y x deja de existir
    // si trataramos de imprimir la variable x, el compilador nos daría un error
    // impidiéndonos compilar
}
```