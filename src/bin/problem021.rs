use project_euler::proper_divisors;

fn main() {
    println!(
        "Problem 021: {}",
        (1..10_000)
            .filter(|&n| {
                let sum = proper_divisors(n).iter().sum::<u64>();
                sum != n && proper_divisors(sum).iter().sum::<u64>() == n
            })
            .sum::<u64>(),
    )
}
