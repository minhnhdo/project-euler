use num::bigint::BigUint;
use num::traits::One;
use std::collections::HashMap;
use std::{cmp, iter};

pub fn primes_iter() -> impl Iterator<Item = u64> {
    iter::once(2).chain((3..).filter(|n| {
        n % 2 != 0
            && (3..=(*n as f64).sqrt() as u64)
                .step_by(2)
                .all(|i| n % i != 0)
    }))
}

pub fn largest_prime_factor(mut n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    for p in primes_iter() {
        while n % p == 0 {
            n /= p;
        }
        if n == 1 {
            return p;
        }
    }
    panic!();
}

pub fn is_palindrome(s: &str) -> bool {
    s.chars().zip(s.chars().rev()).all(|(c1, c2)| c1 == c2)
}

pub fn prime_factors(mut n: u64) -> HashMap<u64, u32> {
    let mut ret = HashMap::new();
    if n <= 1 {
        return ret;
    }
    for p in primes_iter() {
        while n % p == 0 {
            n /= p;
            let count = ret.entry(p).or_insert(0);
            *count += 1;
        }
        if n == 1 {
            return ret;
        }
    }
    ret
}

pub fn char_digit_to_u8(d: char) -> u8 {
    d as u8 - b'0'
}

pub fn number_of_factors(n: u64) -> u64 {
    prime_factors(n).values().map(|e| *e as u64 + 1).product()
}

pub fn collatz_iter(mut n: u64) -> impl Iterator<Item = u64> {
    iter::from_fn(move || {
        if n == 1 {
            return None;
        }
        let ret = n;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        Some(ret)
    })
}

pub fn k_combinations(n: &BigUint, k: &BigUint) -> BigUint {
    num::range(n - k + BigUint::one(), n + BigUint::one()).product::<BigUint>()
        / num::range(BigUint::one(), k + BigUint::one()).product::<BigUint>()
}

pub fn triangle_row_iter<I>(data: &[I]) -> impl Iterator<Item = &[I]> {
    let mut current_index = 0;
    let mut current_length = 1;
    iter::from_fn(move || {
        if current_index + current_length > data.len() {
            return None;
        }

        let ret = &data[current_index..current_index + current_length];
        current_index += current_length;
        current_length += 1;
        Some(ret)
    })
}

pub fn largest_triangular_route_sum(data: &[u32]) -> u64 {
    let mut previous_sums = Vec::new();
    for row in triangle_row_iter(data) {
        let length = row.len();
        let mut sums = Vec::with_capacity(length);
        for (i, &n) in row.iter().enumerate() {
            let left = if i > 0 { previous_sums[i - 1] } else { 0 };
            let up = if i < length - 1 { previous_sums[i] } else { 0 };
            sums.push(cmp::max(left, up) + n as u64);
        }
        previous_sums = sums;
    }
    *previous_sums.iter().max().unwrap()
}
