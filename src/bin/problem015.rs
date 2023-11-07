use num::BigUint;

fn main() {
    println!(
        "Problem 015: {}",
        project_euler::k_combinations(&BigUint::from(40u32), &BigUint::from(20u32))
    );
}
