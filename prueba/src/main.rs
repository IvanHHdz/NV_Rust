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