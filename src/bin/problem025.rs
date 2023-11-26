use num::bigint::BigUint;
use num::traits::One;
use num::traits::Zero;

fn main() {
    println!(
        "Problem 025: {:?}",
        project_euler::fibonacci_iter(BigUint::zero(), BigUint::one())
            .position(|n| n.to_string().len() >= 1_000),
    );
}
