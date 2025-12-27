use wasm_bindgen::prelude::*;
use web_sys::js_sys::BigInt;

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> BigInt {
    if n <= 2 {
        BigInt::from(n)
    }
    else {
        let mut a = BigInt::from(0);
        let mut b = BigInt::from(1);
        for _ in 2..=n {
            let temp = &a + &b;
            a = b;
            b = temp;
        }
        b
    }
}
