fn main() {
    println!(
        "Problem 017: {}",
        (1..=1000)
            .map(|n| {
                say_number::say(n)
                    .chars()
                    .map(|c| if c == ' ' { 0 } else { 1 })
                    .sum::<u32>()
            })
            .sum::<u32>()
    )
}
