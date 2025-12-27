export function fibonacci_js(n) {
    if (n <= 1) return BigInt(n);

    let a = 0n;
    let b = 1n;
    for (let i = 2; i <= n; i++) {
        let temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}