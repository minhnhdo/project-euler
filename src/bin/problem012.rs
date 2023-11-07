fn main() {
    println!(
        "Problem 012: {}",
        (1..)
            .scan(0, |sum, n| {
                *sum += n;
                Some(*sum)
            })
            .skip_while(|n| project_euler::number_of_factors(*n) <= 500)
            .next()
            .unwrap()
    );
}
