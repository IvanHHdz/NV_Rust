# Patterns

Los patterns, o patrones, son una sintaxis especial en Rust. Estos sirven para simplificar o manejar de manera distinta ciertas cosas en nuestros programas. Anteriormente ya hemos utilizado patterns, y ahora vamos a profundizar en ellos de mejor manera. 

Existen muchos lugares y formas de utilizar patterns, algunas de las cuales ya conocemos y otras que serán nuevas. 

# Patterns con `match`

Es una de las más comunes. 

En los brazos del match, sigue la sintaxis:
```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Por ejemplo, como ya los hemos visto antes:
```rust
fn main(){
    let valor = Some(23);
    match valor {
        Some(v) => println!("El valor es {v}."),
        None => println!("No hay valor.")
    }
}
```

O con un enum definido por el usuario:
```rust
fn main(){
    let mi_continente = Continente::America;
    match mi_continente {
        Continente::America => println!("Estoy en América"),
        Continente::Europa => println!("Estoy en Europa"),
        Continente::Asia => println!("Estoy en Asia"),
        Continente::Africa => println!("Estoy en África"),
        Continente::Oceania => println!("Estoy en Oceanía"),
        Continente::Antartida => println!("Estoy en Antártida"),
    }
}

enum Continente {
    America,
    Europa,
    Asia,
    Africa,
    Oceania,
    Antartida
}
```
Como podemos ver, y como ya sabemos, el pattern en los brazos del match es exhaustivo, lo que quiere decir que debemos verificar cada caso del match. Sin embargo, es posible no especificar todos, utilizando un `_`.
```rust
fn main(){
    let mi_continente = Continente::America;
    match mi_continente {
        Continente::America => println!("Estoy en América"),
        _ => println!("No estoy en América"),
    }
}
```

Además, podemos utilizar el `match` con valores literales, de manera similar a como funcionan los `switch` en otros lenguajes como C++:
```rust
fn main() {
    let x = 1;

    match x {
        1 => println!("Primero"),
        2 => println!("Segundo"),
        3 => println!("Tercero"),
        _ => println!("El resto"),
    }
}
```

Esto también funciona especificando valores dentro un enum, por ejemplo:
```rust
fn main(){
    let v = Some(23);
    match v {
        Some(23) => println!("Obtuviste un 23, es impresionante"),
        Some(v) => println!("Obtuviste un {}", v),
        None => println!("No obtuviste nada!")
    }
}
```

Y de la misma forma, igualmente podemos tene multiples patterns para el mismo brazo del match:
```rust
fn main(){
    let v = Some(23);
    match v {
        Some(23) | Some(32) => println!("Obtuviste un 23 o 32, es impresionante"),
        Some(v) => println!("Obtuviste un {}", v),
        None => println!("No obtuviste nada!")
    }
}
```

Funciona igual para literales.

Incluso podemos usar rangos:
```rust
fn main(){
    let v = Some(23);
    match v {
        Some(0..=18) => println!("Obtuviste un 23 o 32, es impresionante"),
        Some(v) => println!("Obtuviste un {}", v),
        None => println!("No obtuviste nada!")
    }
}
```
Siendo similar para literales:
```rust
fn main(){
    let v = 23;
    match v {
        0..=18 => println!("Obtuviste un 23 o 32, es impresionante"),
        v => println!("Obtuviste un {}", v),
    }
}
```

Y también con tuplas:
```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

Además, podemos usar codiciones para que los match sean más específicos. Por ejemplo:
```rust
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("{x} es par"),
        Some(x) => println!("{x} no es par"),
        None => (),
    }
}
```

Por último, podemos usar el operador `@` para declarar un valor que estamos probando. Con un ejemplo se entiende mejor:
```rust
fn main(){
    let usuario = Persona {
        nombre: String::from("Ivan"),
        edad: 20,
    };
    match usuario {
        Persona { nombre, edad: e @ 18..=25 } => {
            println!("Hola {}, eres un adulto joven de {} años.", nombre, e);
        }
        Persona { nombre, edad: e @ 0..=17 } => {
            println!("Hola {}, eres menor de edad, solo tienes {} años.", nombre, e);
        }
        Persona { nombre, edad: e @ 26..=65 } => {
            println!("Hola {}, eres un adulto de {} años.", nombre, e);
        }
        Persona { nombre, edad: e @ 66..=120 } => {
            println!("Hola {}, eres un adulto mayor de {} años.", nombre, e);
        }
        _ => {
            println!("Hola, no se pudo determinar tu grupo de edad.");
        }
    }
}

struct Persona {
    nombre: String,
    edad: u8,
}
```

# Patterns con `if`

Anteriormente vimos también cómo usar la expresión `if let`, la cual puede verse como el equivalente a un brazo de match. Esta de igual forma puede ser encadenada con `else`, incluso con `else if let`. Por ejemplo, usando el mismo enum de antes:
```rust
fn main(){
    let mi_continente = Some(Continente::America);
    if let Some(c) = mi_continente {
        if let Continente::America = c {
            println!("Soy de América!");
        }
        else {
            println!("No soy de América :(");
        }
    }
    else {
        println!("No soy de América :(");
    }
}
```
Y lo anterior podemos simplificarlo más todavía:
```rust
fn main(){
    let mi_continente = Some(Continente::America);
    if let Some(Continente::America) = mi_continente {
        println!("Soy de América!");
    }
    else {
        println!("No soy de América :(");
    }
}
```
O añadir otras condiciones:
```rust
fn main(){
    let mi_continente = Some(Continente::America);
    if let Some(Continente::America) = mi_continente {
        println!("Soy de América!");
    }
    else if let Some(Continente::Europa) = mi_continente {
        println!("Soy Europeo");
    }
    else {
        println!("No soy de América ni de Europa :(");
    }
}
```

O utilizar otros condicionales. 

# Patterns en variables 

En sí, se puede interpretar que la expresión `let Some(Continente::America) = mi_continente` devuelve, por así decirlo, un valor booleano. En cuyo caso, es reemplazable en cualquier forma que otra operación del mismo tipo (una expresión booleana). Y ese pattern puede verse como:
```
let PATTERN = VARIABLE
```

O de forma más general, se entiende que:
```
let PATTERN = EXPRESSION;
```
Que es la misma forma de declarar variables. Claro que para declarar variables hay algunas reglas importantes. Por ejemplo:
```rust
fn main(){
    let Some(x) = Some(1);
}
```
Esto no funciona, claro. Pero es importante mencionar porqué no funciona: porque si la expresión fuera un `None` no se sabría qué hacer. En su lugar, podemos poner un `else`. Pero si hacemos eso, debemos **terminar** ahí, es decir, usar un `return` o un `panic!()`. Esto porque se ha asumido que lo de antes funcionaría: no funcionó, toca hacer algo:
```rust
fn main(){
    let Some(x) = Some(1) else {
        return;
    };
    println!("El valor es {x}");
}
```

Ahora sí funciona, y si fuera un `None` seguiría funcionando, con la diferencia que el programa terminaría en el `return`:
```rust
fn main(){
    let Some(x): Option<i32> = None else {  // debemos especificar el tipo, pues el compilador no puede inferirlo
        return;     // <-- aquí terminaría
    };
    println!("El valor es {x}");
}
```

Sí quisieramos usar un valor por defecto, sería de forma distinta:
```rust
fn main(){
    let Some(x): Option<i32> = (
        if let Some(v) = None {
            Some(v)
        }
        else {
            Some(1)
        }
    )
    else {
        return; // <-- siempre debemos colocarlo
    };
    println!("El valor es {x}");
}
```

# Patterns con `while`

Como mencionábamos antes, `let PATTERN = VARIABLE` puede verse como una expresión que retorna un valor booleano. Y como `while` utiliza una expresión booleana para su bucle, pues es válido también.

Por ejemplo:
```rust
fn main(){
    let mut mi_continente = Some(Continente::America);
    let mut contador = 0;
    while let Some(Continente::America) = mi_continente {
        println!("Estamos en América");
        if contador > 10 {
            mi_continente = None;
        }
        contador += 1;
    }
}
```
Es completamente válido, e imprimirá 10 veces `"Estamos en América"`. Claro que no es el úso más práctico, pero es una forma. Y hay ocasiones en que sí es práctico.

# Desestructuración
Antes habíamos visto la desestructuración de tuplas, por ejemplo:
```rust
fn main(){
    let p = (2, 3);
    let (x, y) = p;
}
```

Sin embargo, es importante mencionar que esto también funciona con estructuras:
```rust
fn main(){
    let car = Carro {
        marca: String::from("Toyota"),
        color: String::from("Rojo"),
        year: 2009
    };

    let Carro{marca: m, color: c, year: y} = car;

    println!("Es un carro {m} de color {c} año {y}");
}

struct Carro {
    marca: String,
    color: String,
    year: i32,
}
```

Esto mismo nos sirve, por ejemplo, con match:
```rust
fn main(){
    let p = Point{x: 23, y: 32};

    match p {
        Point { x:0, y:0 } => println!("Origen"),
        Point { x, y: 0 } => println!("Intercepto en y"),
        Point { x:0 , y } => println!("Intercepto en x"),
        Point { x, y } => println!("Punto en ({}, {})", x, y)
    }
}

struct Point {
    x: i32,
    y: i32
}
```