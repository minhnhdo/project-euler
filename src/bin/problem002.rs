use std::iter;

fn main() {
    let mut a = 0;
    let mut b = 1;
    let fib_iter = iter::from_fn(move || {
        let ret = a + b;
        a = b;
        b = ret;
        Some(ret)
    });
    println!(
        "Problem 002: {}",
        fib_iter
            .take_while(|n| n < &4_000_000)
            .filter(|n| n % 2 == 0)
            .sum::<u64>()
    );
}
