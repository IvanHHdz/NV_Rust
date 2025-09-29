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

