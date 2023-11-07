fn main() {
    println!(
        "Problem 010: {}",
        project_euler::primes_iter()
            .take_while(|p| *p < 2_000_000)
            .sum::<u64>()
    );
}
