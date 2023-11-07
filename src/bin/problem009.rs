pub fn problem009() -> u64 {
    for c in (1..=1000).rev() {
        for b in 1..(1000 - c) {
            let a = 1000 - b - c;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    panic!();
}

fn main() {
    println!("Problem 009: {}", problem009());
}
