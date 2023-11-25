fn main() {
    let mut result = None;
    let mut p = project_euler::permutations((0..10).collect::<Vec<_>>());
    for _ in 1..=1_000_000 {
        result = p.next();
    }
    println!(
        "Problem 024: {:?}",
        result.map(|v| v.into_iter().map(|i| i.to_string()).collect::<String>())
    );
}
