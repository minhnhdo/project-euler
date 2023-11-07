fn main() {
    println!(
        "Problem 007: {}",
        project_euler::primes_iter().skip(10_000).next().unwrap()
    );
}
