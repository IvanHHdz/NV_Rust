# Condicionales

Como en otros lenguajes de programación, en Rust tenemos las condicionales `if` y `else`. Las cuales funcionan exactamente igual que en el resto de lenguajes:
```rust
fn main(){
    let a = 10;

    if a < 10 {
        println!("Menor a 10");
    }
    else if a == 10 {
        println!("Igual a 10");
    }
    else {
        println!("Mayor a 10");
    }
}
```

Y claro, la condición debe ser (o producir) un valor estríctamente booleano. Los `else` son opcionales también. 

Como mencionamos, funcionan exactamente igual que en otros lenguajes.

Ahora, una pequeña particularidad: las expresiones `if` dentro de sentencias `let`. Para entender esto con claridad, hay que entender que en Rust se hace una distinción clara entre **expresión** y **sentencia**. Esta radica en el hecho que una expresión puede retornar un valor, una sentencia no.

Una forma de diferenciar una sentencia y una expresión es mediante el `;`, pues solo se usa en las sentencias. Y ciertas líneas, como poner un entero solo, puede ser sentencia o expresión dependiendo si se le coloca o no `;`.

Al considerar a los condicionales (`if` y `else`) como expresiones, podemos entonces entender que pueden retornar valores. Y estos valores, claro, pueden ser guardados dentro de variables.

```rust
fn main(){
    let condicion = true;
    let variable = if condicion {1} else {2};
    println!("variable {variable}");
    // imprimirá 1
}
```

Pero claro, visto así no tiene mucha diferencia a cosas como el condicional ternario de lenguajes como Java, C++ o Python. Pero sí tiene diferncia: En Rust puede ser un bloque completo de código.

```rust
fn main(){
    let condicion1 = true;
    let condicion2 = false;
    let condicion3 = 10;
    let variable = 
        if condicion1 {
            if condicion3 % 3 == 0{ 
                0
            }
            else {
                condicion3
            }
        } else {
            if condicion2 {
                condicion3 + 1
            }
            else {
                1
            }
        };
    println!("variable {variable}");
    // imprimirá 10
}
```

Claro, para que todo esto funciones se deben considerar todos los casos, es decir, sí es necesario el `else` aquí. Y todas las expresiones deben ser del mismo tipo (en este ejemplo, todas son `i32`) o el compilador nos impedirá compilar el programa.

Y de igual forma, es perfectamente válido usar expresiones dentro de las condiciones, siempre que retornen valores booleanos. Lo que, como dijimos antes, incluye también bloques `if - else`:
```rust
fn main(){
    let condicion1 = true;
    let condicion2 = false;
    let condicion3 = 10;
    let variable = 
        if condicion1 {
            if if !condicion2 {condicion3 == 10} else {false} { 
                -1
            }
            else {
                condicion3 + 1
            }
        } else {
            1
        };
    println!("variable {variable}");
    // imprimirá -1
}
```

Claro, esto no es exclusivo de Rust, pero tampoco es posible en lenguajes como C++, Java, JavaScript, Python, etc. Aunque sí es posible en lenguajes como Kotlin, Haskell y Ruby.

# Bucles

Y al igual que en otros lenguajes, podemos llegar a tener la necesidad de usar bucles. Rust tiene 3: `while`, `for` y `loop`.

## `while`

`while` funciona igual que en otros lenguajes como C++:
```rust
fn main() {
    let mut num = 3;

    while num != 0 {
        println!("{number}!");

        num -= 1;
    }
}
```

De igual forma, tenemos también el `break` y el `continue`:
```rust
fn main() {
    let mut num = 3;

    while num > 0 {
        println!("{number}!");
        num -= 1;
        if num == 0 {
            break;
        }
        else {
            continue;
        }
    }
}
```

Una cosa interesante que posee Rust también es la capacidad de darle una etiqueta a los bucles, para salir en caso que sea necesario:
```rust
fn main(){
    let mut i = 0;
    // llamamos este bucle 'outer
    'outer: while i < 3 {
        let mut j = 0;
        while j < 3 {
            println!("i = {}, j = {}", i, j);
            if i == 1 && j == 1 {
                // salimos del bucle 'outer, es decir, de todo el bucle
                break 'outer;
            }
            j += 1;
        }
        i += 1;
    }
    println!("Fin");
}
```

## `for`

En cuanto a `for`, es igual que en Python:
```rust
fn main() {
    // el (1..=10) es la forma que tiene Rust para hacer una lista de número de 1 a 10
    // el = es para especificar que también incluye el 10
    // funciona igual al range() de python
    for num in (1..=10) {
        println!("{num}!");
    }
}
```

Y al igual que el `while`, es posible darles una etiqueta.

## `loop`

Y por último: el `loop`. Este es, básicamente, un `while true`. Lo que lo hace especial es su capacidad de retornar valores mediante `break`:
```rust
fn main() {
    let mut t = 0;
    let resultado = loop {
        t += 1;
        if t == 10 {
            break t * 5;
        }
    };
    println!("Resultado {resultado}");
    // Resultado 50
}
```

Y de la misma forma, es posible darle etiquetas al igual que el resto de bucles:
```rust
fn main() {
    let mut t = 0;
    'contando: loop {
        println!("t = {t}");
        let mut t2 = 10;
        loop {
            println!("t2 = {t2}");
            if t2 == 9 {
                break;
            }
            if t == 2 {
                break 'contando;
            }
            t2 -= 1;
        }
        t += 1;
    }
    println!("Al final, t = {t}");
    // Al final, t = 2
}
```
