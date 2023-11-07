use std::{cmp, collections::HashMap};

fn main() {
    println!(
        "Problem 005: {}",
        (2..=20)
            .fold(HashMap::new(), |mut acc, n| {
                project_euler::prime_factors(n).iter().for_each(|(p, c)| {
                    let e = acc.entry(*p).or_insert(*c);
                    *e = cmp::max(*e, *c);
                });
                acc
            })
            .iter()
            .fold(1, |acc, (p, c)| acc * p.pow(*c))
    );
}
