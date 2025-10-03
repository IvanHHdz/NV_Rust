# Asincronía
Como sabemos, existen multiples formas de hacer que nuestro programa realice multiples tareas "al mismo tiempo". Una de ellas es el uso de hilos, que fue cubierta en el bloque 1. Por otro lado, tenemos también la programación asincrónica. En esta, a diferencia de la programación con hilos, el sistema operativo no se ve involucrado, sino que todo ocurre dentro del propio programa. Esto provee ciertas ventajas, y a su vez trae consigo multiples complejidades nuevas.

Una de las ventajas de este enfoque es la rapidez, pues puede realizar los cambios de manera más rápida y eficiente.

Por el momento, veamos un ejemplo clásico:
```rust
// definimos una función asincrónica
async fn say_hello() {
    println!("hello, world!");
}

#[tokio::main]
async fn main() {
    // llamamos la función y esperamos por el resultado
    say_hello().await;
}
```

Para este ejemplo es importante recordar importar `tokyo` en el `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.40.0", features = ["full"] }
```

# Future

Un concepto central dentro de la programación asincrónica de Rust es el de Future. Un future es cualquier estructura (como struct, enum, etc) que implementa el trait de `Future`. Este trait nos permite trabajar con el dato aún cuando no lo tenemos todavía. Funcionando de manera similar a como funcionan las promesas en lenguajes como JavaScript.

De momento no tocaremos mucho el concepto. Sin embargo, más adelante lo veremos a detalle junto a los _big futures_, que son combinaciones para crear tareas de rust, que son los future al ejecutarse. 

# `async` y `await`

`async` es una palabra reservada de Rust que nos ayuda a indicar que una función es asincrónica. Dentro de una función asincrónica, podemos usar `await`, de la misma forma que al llamar una función asincrónica podemos (y debemos, tarde o temprano) usar `await`. `await` obliga a esperar el resultado de la función asincrónica, es decir, el valor del Future que esta retorne, puesto que las funciones asincrónicas retornan Futures (un objeto que representa un valor que estará disponible en el futuro).

Veamos un ejemplo, estas son funciones asincrónicas:
```rust
// esta función es asincrónica, aunque no tiene ninguna necesidad de serlo
async fn add(a: u32, b: u32) -> u32 {
    a + b
}

// esta función es asincrónica, hace lo mismo, pero tiene que esperar para hacer la suma
async fn wait_to_add(a: u32, b: u32) -> u32 {
    // pausamos la ejecución por 5 seg
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    a + b
}
```

Si llamamos las funciones desde un `main` deberemos usar `await` para obtener el valor. Caso contrario, obtendremos un Future, el cual si usamos `await` podremos obtener el valor (una vez que se ejecute el código).

Para poder visualizar mejor esto, veamos un ejemplo comparativo con hilos.

**Asincrónico:**

```rust
#[tokio::main] 
async fn main() {
    // declaramos una variable `resultado` la cual tendrá el resultado de la función suma_lenta()
    // esta función es asincrónica, por lo que retornará un future
    // este lo podemos aplicar `.await` ahora o después (lo aplicaremos después)
    let resultado = suma_lenta(5, 10);  // como no usamos `.await`, el programa sigue
    // esperamos un segundo
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Hola, mundo!");           // imprimirá esto
    // ahora usaremos el `.await`
    // por lo que se empezará a ejecutar la función `suma_lenta()`
    // el main se detiene aquí, esperando que se obtenga el valor de `resultado`
    println!("El resultado es: {}", resultado.await);
}

// esta es la función 
async fn suma_lenta(a: i32, b: i32) -> i32 {
    // imprimimos algo para ver cuándo se empieza a ejecutar
    println!("Calculando la suma de {} y {}...", a, b);
    // hacemos esperar 5 seg
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    a + b   // retornamos el valor
}
// imprimirá:
/*  
    Hola, mundo!
    Calculando la suma de 5 y 10...
    El resultado es: 15
*/
```

**Hilos:**

```rust
use std::{sync::mpsc::channel, thread::sleep};

fn main() {
    let (tx, rx) = channel();       // crearemos un canal para hacer el retorno
    // ejecutamos un hilo, movemos tx al hilo para enviar el resultado
    let hilo = std::thread::spawn(move || {
        // ejecutamos le suma_lenta() y enviamos el resultado
        tx.send(suma_lenta(5, 10)).unwrap();
    });
    // esperamos un segundo
    sleep(std::time::Duration::from_secs(1));
    // imprimimos esto
    println!("Hola, mundo!");
    // esperamos por el resultado para imprimirlo
    println!("El resultado es: {}", rx.recv().unwrap());
    hilo.join().unwrap();   // esperamos por si el hilo no ha terminado
}

// la función
fn suma_lenta(a: i32, b: i32) -> i32 {
    // imprimimos para saber cuando empieza a ejecutarse la función (que resulta ser todo el hilo)
    println!("Calculando la suma de {} y {}...", a, b);
    // hacemos esperar 5 seg
    sleep(std::time::Duration::from_secs(5));
    a + b   // retornamos el valor
}
// imprimirá:
/*
    Calculando la suma de 5 y 10...
    Hola, mundo!
    El resultado es: 15
*/
```

> Nota, es posible que el resultado de la versión con hilos no sea así siempre, debido a ciertos aspectos referentes al planificador de hilos del sistema operativo (scheduler) y la carga del sistema. Pero en general, debería verse así.

Este ejemplo es útil para ilustrar un detalle importante: Cómo se ejecuta cada ejemplo.

El ejemplo con `async`, como podemos ver, se ejecuta todo el `main` hasta llegar a la parte con `.await`, donde empieza a ejecutarse la función `suma_lenta()`. No se está ejecutando al mismo tiempo, sino que están **tomando turnos** para ejecutarse. Y quien organiza esos turnos es el propio programa (con `.await`) y el runtime de `tokyo`.

Por otro lado, el ejemplo con hilos, podemos ver que mientras se espera al segundo que se da, se ejecuta una parte de la función `suma_lenta()`, para luego esperar los 5 segundos. Mientras espera, se ejecuta parte del main hasta que se bloquea esperando el valor por el canal. Pueden estar tomando turnos, o **ejecutándose paralelamente** (eso depende del sistema operativo y de la máquina), pero de eso no se encarga el programa, sino el sistema operativo. 

# Lanzar tareas

Sin embargo, es importante tener en mente que, usar la asincronía de la forma anterior no es lo más ideal. Pues no estamos aprovechando el tiempo de espera (en estos casos, los momentos que se usa `sleep`), mientras que con hilos sí la estamos aprovechando. Para lograr esto, debemos hacerlo de forma distinta: lanzando las tareas. Para esto, debemos hacer uso de la función `spawn` de Tokyo, esto debido a que las tareas (tasks) son conceptos propios del runtime.

Por ejemplo, el ejemplo anterior nos quedaría:
```rust
use tokio::spawn;

#[tokio::main] 
async fn main() {
    let resultado = spawn(suma_lenta(5, 10));
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Hola, mundo!");
    println!("El resultado es: {}", resultado.await.unwrap());  // ahora el .await funciona como el .join de hilos
}

// esta es la función 
async fn suma_lenta(a: i32, b: i32) -> i32 {
    println!("Calculando la suma de {} y {}...", a, b);
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    a + b 
}
// imprimirá:
/*  
    Calculando la suma de 5 y 10...
    Hola, mundo!
    El resultado es: 15
*/
```

Notemos que ahora parece estarse ejecutando como se ejecutaba con hilos. En realidad, está haciendo lo mismo. La diferencia radica en el hecho que, quien se encarga de organizar cuales tareas tendrán hilos del sistema (si el sistema puede tener hilos) es el runtime de Tokyo. Esto, si bien parece en principio igual que lanzar hilos, tiene ciertas ventajas: algunas tareas (las que no se lanzaron en hilos del sistema) tendrán tiempos de respuesta más cortos, además que sigue manteniendo cierta concurrencia incluso en sistemas que no soporten hilos.