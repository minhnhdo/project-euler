fn main() {
    let s = (1..=100).sum::<u32>();
    println!(
        "Problem 006: {}",
        s * s - (1..=100).map(|n| n * n).sum::<u32>()
    );
}
