#[tokio::main] 
async fn main() {
    // declaramos una variable `resultado` la cual tendrá el resultado de la función suma_lenta()
    // esta función es asincrónica, por lo que retornará un future
    // este lo podemos aplicar `.await` ahora o después (lo aplicaremos después)
    let resultado = suma_lenta(5, 10);  // como no usamos `.await`, el programa sigue
    // esperamos un segundo
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