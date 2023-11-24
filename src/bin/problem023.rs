use project_euler::abundant_numbers_iter;
use std::collections::HashSet;

fn main() {
    const LIMIT: u64 = 28123;
    let numbers = abundant_numbers_iter()
        .take_while(|&n| n < LIMIT)
        .collect::<Vec<_>>();
    let mut sum_of_two_abundant_numbers = HashSet::new();
    for i in numbers.iter() {
        for j in numbers.iter().take_while(|&&j| j <= LIMIT - i) {
            sum_of_two_abundant_numbers.insert(i + j);
        }
    }
    println!(
        "Problem 023: {}",
        (1..LIMIT)
            .filter(|n| !sum_of_two_abundant_numbers.contains(n))
            .sum::<u64>(),
    );
}
