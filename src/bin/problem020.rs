use num::BigUint;

fn main() {
    println!(
        "Problem 020: {}",
        (2u8..=100)
            .fold(BigUint::from(1u8), |acc, x| acc * x)
            .to_str_radix(10)
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .sum::<u32>()
    );
}
