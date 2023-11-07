fn main() {
    println!(
        "Problem 014: {}",
        (1..1_000_000)
            .max_by_key::<u64, _>(|n| project_euler::collatz_iter(*n).map(|_| 1).sum())
            .unwrap()
    );
}
