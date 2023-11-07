fn main() {
    println!(
        "Problem 004: {}",
        (100..1000)
            .rev()
            .map(|n| (100..=n).rev().map(move |m| n * m))
            .flatten()
            .filter(|p| project_euler::is_palindrome(&p.to_string()))
            .max()
            .unwrap()
    );
}
