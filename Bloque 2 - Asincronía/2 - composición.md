# Composición de Futures concurrentes

Por composición nos referimos a unir o combinar los resultados de varias tareas. En nuestro caso, veremos cómo podemos hacer que dos o más tareas se ejecuten de manera cocnurrente (pero no paralela) compartiendo tiempo para poder obtener un resultado.

Contamos con dos formas de lograr esto: `select`/`race` y `join`. En el primero, las tareas compiten entre sí, la primera que termina devuelve el resultado y el resto se cancelan; el segundo caso se espera a que todas terminen, obteniendo el valor de retorno de todas.

Es importante destacar otra vez que **no se ejecutan en paralelo**, sino que todas las tareas que se envían así se ejecutan concurrentemente en intervalos de tiempo definidos por el runtime (de Tokyo en este caso) en el mismo hilo del sistema. Esto es importante porque nos permite hacer exactamente lo mismo en un entorno que no soporte hilos, pues el sistema no se entera de nada.

# `join`

Primero veamos `join`. Como mencionábamos, este ejecuta todas las tareas concurrentemente en el mismo hilo, y espera a que todas terminen para poder brindar el retorno de todas. Veamos un ejemplo:
```rust
use tokio::join;

#[tokio::main]
async fn main() {
    let magnitud = 25f64;
    let angulo = 1.031448;

    println!("Calculando...");

    let (x, y) = join!(calcular_x(magnitud, angulo), calcular_y(magnitud, angulo));

    println!("Las coordenadas para una magnitud {magnitud} y un angulo {angulo} son ({x}, {y})");
}

async fn calcular_x(m: f64, a: f64) -> f64 {
    println!("Calculando x...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    m * a.cos()
}

async fn calcular_y(m: f64, a: f64) -> f64 {
    println!("Calculando y...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    m * a.sin()
}
```

Notemos que a `join` no le importa si alguno de las tareas falla retornando un `Err`. Lo único que lo podría detener es un `panic`, ya que mataría el hilo completo. Veamos un ejemplo con `Result`:
```rust
use tokio::join;

#[tokio::main]
async fn main() {
    let magnitud = 25f64;
    let angulo = 1.031448;

    println!("Calculando...");

    let resultado = join!(calcular_x(magnitud, angulo), calcular_y(magnitud, angulo));

    if let (Ok(x), Ok(y)) = resultado {
        println!("Las coordenadas para una magnitud {magnitud} y un angulo {angulo} son ({:?}, {:?})", x, y);
    }
    else {
        println!("Ha ocurrido un error");
        println!("Aunque sí se ha evaluado x = {}", resultado.0.unwrap());
    }
}

async fn calcular_x(m: f64, a: f64) -> Result<f64, String> {
    println!("Calculando x...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(m * a.cos())
}

async fn calcular_y(m: f64, a: f64) -> Result<f64, String> {
    println!("Calculando y...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    return Err("Error calculando y".to_string());
    Ok(m * a.sin())
}
```

Sin embargo, esto se puede simplificar utilizando `try_join`, que funciona igual que `join`, con la diferencia que este sí cancela todo en caso de error:


```rust
use tokio::try_join;

#[tokio::main]
async fn main() {
    let magnitud = 25f64;
    let angulo = 1.031448;

    println!("Calculando...");

    let resultado = try_join!(calcular_x(magnitud, angulo), calcular_y(magnitud, angulo));

    if let Ok((x, y)) = resultado {
        println!("Las coordenadas para una magnitud {magnitud} y un angulo {angulo} son ({:?}, {:?})", x, y);
    }
    else {
        println!("Ha ocurrido un error");
    }
}

async fn calcular_x(m: f64, a: f64) -> Result<f64, String> {
    println!("Calculando x...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(m * a.cos())
}

async fn calcular_y(m: f64, a: f64) -> Result<f64, String> {
    println!("Calculando y...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    return Err("Error calculando y".to_string());
    Ok(m * a.sin())
}
```

# `race/select`
De manera similar a `join`, se lanzan varias tareas de manera concurrente (no paralela). La diferencia es que no se espera a que todas las tareas terminen, sino al terminar una las demás se cancelan. Veamos un ejemplo:
```rust
use tokio::select;

#[tokio::main]
async fn main() {
    let resutado = select! {
        res = suma_lenta(1, 2) => res,
        res = suma_super_lenta(3, 4) => res,
    };
}

async fn suma_lenta(a: i32, b: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    a + b
}

async fn suma_super_lenta(a: i32, b: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    a + b
}
```

Como podemos ver, funciona como un `match`. De hecho, acepta patterns, de la misma manera que un `match` o un `if let`. 
```rust
use tokio::select;

#[tokio::main]
async fn main() {
    let resultado = select! {
        Some(res) = suma_lenta(1, 2) => res,
        res = suma_super_lenta(3, 4) => res.unwrap(),
    };
}

async fn suma_lenta(a: i32, b: i32) -> Option<i32> {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    Some(a + b)
}

async fn suma_super_lenta(a: i32, b: i32) -> Option<i32> {
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    Some(a + b)
}
```