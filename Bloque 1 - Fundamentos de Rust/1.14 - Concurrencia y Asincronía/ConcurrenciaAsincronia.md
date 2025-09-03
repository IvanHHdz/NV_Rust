# _Fearless Concurrency_

Otro de los puntos fuertes de Rust es precisamente su capacidad de facilitar la programación paralela y concurrente. Ayudando a disminuir los errores que se pueden presentar en esta parte.

En un principio, el equipo de Rust creía que asegurar la seguridad de la memoria y prevenir los problemas de la concurrencia eran cosas separadas y que necesitaban resolverse por separado. Con el tiempo se dieron cuenta que esto no era así. Resulta que el [_Ownership_](../1.4%20-%20Ownership/Ownership.md) y sus reglas son de gran ayuda para manejar también los errores de concurrencia. De esta forma, muchos de los errores de concurrencia se vuelven errores en tiempo de compilación, lo que facilita su corrección. A esto se le ha dado el apodo de _fearless concurrency_ (concurrencia sin miedo).

Rust utiliza, en la librería estándar, el modelo de 1:1 para los hilos.

> Nota: por ahora nos referiremos por "concurrencia" a "concurrencia y/o paralelismo".

> Nota: En esta parte vamos a asumir que usted ya sabe qué son los hilos, el multiprocesamiento, paralelismo, concurrencia, etc., junto a los problemas típicos derivados de los anteriores como condición de carrera, interbloqueo, inanición, etc. 

# Hilos

La librería estandar de Rust utiliza un modelo 1:1 de hilos, lo que quiere decir que cada hilo de Rust es un hilo del sistema. 

Para crear un nuevo hilo utilizamos la función `thread::spawn()` y le pasamos una closure como parámetro, el código de dicha closure es el código que ejecutará el nuevo hilo. Un ejemplo sencillo:
```rust
use std::thread;            // recordemos que debemos importarlo de la lirería estandar
use std::time::Duration;    // esto es para esperar o pausar una ejecución

fn main() {
    // creamos un nuevo hilo
    thread::spawn(|| {      
        for i in 1..10 {
            println!("Hola desde el nuevo hilo\t({i})");
            thread::sleep(Duration::from_millis(1));
        }
    });
    // esto se ejecuta en el hilo principal
    for i in 1..5 {
        println!("Hola desde el hilo principal\t({i})");
        thread::sleep(Duration::from_millis(1));
    }
}
```
El resultado será un entrelazamiento de ambos hilos. La función `thread::sleep()` sirve para pausar durante un cierto intervalo de tiempo la ejecución de este hilo, lo que provoca (en la mayoría de casos) que otro hilo tome su lugar en la ejecución.

Es importante destacar que, como podemos ver, el hilo principal puede llegar a terminar antes que el nuevo hilo pues el principal solo hace un bucle de 1 a 5. Si esto ocurre, es decir que el hilo principal termina sin que el nuevo termine, el hilo nuevo que creamos también será terminado. Si no queremos que esto ocurra, es decir que queremos que todos los hilos terminen su ejecución, podemos usar el método `.join()` para eso. Pero primero, debemos guardar el valor que retorna la creación del hilo, el cual es un valor de tipo `JoinHandle<T>`.
```rust
use std::thread;
use std::time::Duration;

fn main() {
    // guardamos el valor del nuevo hilo en la variable hilo
    let hilo = thread::spawn(|| {
        for i in 1..10 {
            println!("Hola desde el nuevo hilo\t({i})");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hola desde el hilo principal\t({i})");
        thread::sleep(Duration::from_millis(1));
    }
    // cuando el for del main termine, esperará a que termine el hilo nuevo
    hilo.join().unwrap();
}
```
Ahora bien, supongamos que tenemos una String y deseamos que un nuevo hilo trabaje con la String. Para lograrlo, podemos hacer que la closure capture esa String. Pero para eso, hay algo que debemos tener en cuenta: la closure no solo debe capturar el valor, debe moverlo.

¿Por qué? Veamos el siguiente ejemplo (esto no compila):
```rust
use std::thread;

fn main() {
    let nombre = String::from("Ferris");

    let hilo = thread::spawn(|| {
        println!("Hola {nombre}!");
    });

    drop(nombre);   // hemos liberado la memoria de nombre!

    hilo.join().unwrap();
}
```

La razón por la que esto no compila es principalmente porque la closure del nuevo hilo no está moviendo el _ownership_ de la variable `nombre` dentro de sí, sino solo una referencia inmutable. El compilador nos impedirá compilarlo porque es incapaz de saber (igual que nosotros) el orden que seguirá la ejecución de nuestro código. Lo hace para prevenir que llegue a pasar algo como lo que precisamente ocurre adelante: se utiliza `drop()` sobre la variable `nombre`. Si el compilador nos permitiera hacer esto, sería un grave problema. ¿Qué ocurriría si se ejecuta `drop()` antes que `println!()`? Habrían problemas. Por suerte, el compilador evita esto impidiendonos compilar este codigo inseguro.

Pero, ¿cómo hacemos entonces para que el hilo pueda utilizar una variable? Primero, debemos mover el _ownership_ de la variable dentro de la closure. Segundo, no usar `drop()` ahí, pues sería inválido (el _ownership_ de nombre lo habrémos movido de ahí). Esto lo hacemos de la siguiente manera:
```rust
use std::thread;

fn main() {
    let nombre = String::from("Ferris");

    // utilizamos move para mover la variable
    let hilo = thread::spawn(move || {
        println!("Hola {nombre}!");
    });

    hilo.join().unwrap();
}
```

Y así, [las relgas de _ownership_](../1.05%20-%20Ownership/Ownership.md) nos salvan una vez más de hacer algo que tendría terribles, inesperadas e impredecibles consecuencias sobre nuestro programa.

# Transferencia de información

En muchas ocasiones, necesitarémos que los hilos se envien información entre sí. Rust sigue la idea de lenguajes como Go, que dice:
> “No te comuniques compartiendo la memoria; más bien, comparte la memoria comunicándote”.

Rust implementa esto haciendo uso de canales (_channels_) para permitir que uno o varios hilos envien mensajes a un solo hilo (multiples productores, un consumidor). Esto funciona de manera sencilla: cada canal tiene dos "extremos" estos son el transmisor y receptor. Para comunicarse, un hilo envía un dato por medio del transmisor del canal, y otro hilo recive el dato por el receptor del canal.

Para entenderlo mejor, veamos un ejemplo:
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel(); // con esto creamos el canal

    let hilo1 = thread::spawn(move || {
        let nombre = String::from("Ferris");
        tx.send(nombre).unwrap();   // retorna un Result<T, E>
        // este Result<T, E> será Ok(()) siempre que exista todavía el receptor rx correspondiente
        // sin embargo, si por alguna razón no es válido (fue droppeado, por ejemplo), retornará Err(SendError(valor))
        // por lo que nunca se pierde el valor
        println!("Trabajo hecho!");
    });

    let hilo2 = thread::spawn(move || {
        println!("Esperando nombre");
        let usuario = rx.recv().unwrap(); // también retorna un Result<T, E>
        // pero lo hace con Ok() si se recibió un dato.
        // si no llega nada, se bloquea esperando
        // mientras que un Err() si deja de existir o ser válido todo productor tx
        // también existe .try_recv() que hace exactamente lo mismo, con la diferencia que no esperará
        // retorna Err(TryRecvError::Empty) si la cola estaba vacía y Err(TryRecvError::Disconnected) si ya no hay emisores
        // lo que lo vuelve útil por si el hilo receptor tiene más cosas por hacer.
        println!("Hola {usuario}!");
    });

    hilo1.join().unwrap();
    hilo2.join().unwrap();
}
```

Es importante recalcar que usar `.send()` envía no solo el dato, sino que el _ownership_ de la variable que se pasa. Por lo que, por ejemplo, no podemos volver a usar `nombre` en `hilo1` después de usar el `.send()`.

De la misma forma, podemos hacer esto con multiples hilos productores clonando el `tx`:
```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel(); // con esto creamos el canal
    let tx1 = tx.clone();

    // productor
    let hilo1 = thread::spawn(move || {
        let nombre = String::from("Ferris");
        tx.send(nombre).unwrap();   
        println!("Trabajo 1 hecho!");
    });

    // productor
    let hilo2 = thread::spawn(move || {
        let nombre = String::from("Tux");
        tx1.send(nombre).unwrap();   
        println!("Trabajo 2 hecho!");
    });

    // consumidor
    let hilo3 = thread::spawn(move || {
        loop {
            println!("Esperando nombre");
            let usuario = match rx.recv() { // siempre que llegue aquí se bloqueará esperando
                Ok(valor) => valor,
                Err(_) => { // si no hay más productores, retornará error y terminará
                    println!("No hay más productores!");
                    println!("No hay más que esperar");
                    break
                }
            };
            println!("Hola {usuario}!");
        }
    });

    // Ejemplo de salida
    /*
        Trabajo 2 hecho!
        Trabajo 1 hecho!
        Esperando nombre
        Hola Tux!
        Esperando nombre
        Hola Ferris!
        Esperando nombre
        No hay más productores!
        No hay más que esperar
     */

    hilo1.join().unwrap();
    hilo2.join().unwrap();
    hilo3.join().unwrap();
}
```

Como dato curioso, podemos simplificar el consumidor usando `for`:
```rust
    // consumidor
    let hilo3 = thread::spawn(move || {
        for usuario in rx {
            println!("Esperando nombre");
            println!("Hola {usuario}!");
        }
        println!("No hay más productores!");
        println!("No hay más que esperar");
    });
```

# Exclusión mutua

La exclusión mutua, abreviada como _mutex_, consiste en un mecanismo que asegura que ciertos datos solo sean accesibles por un hilo a la vez. Esto es un mecanismo clásico para poder compartir memoria entre hilos (o procesos en general). Para ello, cada hilo que trate de acceder a la variable necesita asegurarse que nadie más la está usando, y luego bloquearla, para que nadie más pueda usarla; una vez termina, debe desbloquearla, para que otros puedan usarla.

Veamos cómo se utiliza en Rust con un ejemplo de un solo hilo (el principal):
```rust
use std::sync::Mutex;

fn main() {
    // creamos un Mutex<T> con new()
    let game = Mutex::new(String::from("Hollow Knight"));   // en este caso será Mutex<String>

    // podemos imprimirlo
    println!("Actualmente jugando: {game:?}");  // Actualmente jugando: Mutex { data: "Hollow Knight", poisoned: false, .. }

    {
        // a continuación, el hilo (el principal y único) se bloquea hasta que le toca usar el Mutex
        // devuelve un LockResult<MutexGuard<T>>
        // dará un error si, por alguna razón, un hilo que tenía este mutex cracheó (panicked)
        // en cuyo caso se dice que el mutex está "envenenado" (poisoned)
        let mut usuario = game.lock().unwrap();
        *usuario = String::from("Portal");
    }   // cuando llegamos aquí, el MutexGuard se asegura que el mutex vuelva a estar bloqueado
        // esto lo logra por ser un puntero inteligente 

    println!("Actualmente jugando: {game:?}");  // Actualmente jugando: Mutex { data: "Portal", poisoned: false, .. }
}
```



# Programación asincrónica

Cap. 17