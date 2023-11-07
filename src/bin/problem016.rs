use num::traits::Pow;
use num::BigUint;

fn main() {
    println!(
        "Problem 016: {}",
        BigUint::from(2u32)
            .pow(&BigUint::from(1000u32))
            .to_str_radix(10)
            .chars()
            .map(|d| project_euler::char_digit_to_u8(d) as u32)
            .sum::<u32>()
    );
}
