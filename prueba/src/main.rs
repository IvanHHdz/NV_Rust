use tokio::{select, sync::oneshot::channel};

#[tokio::main]
async fn main() {
    let (tx, rx) = channel();
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