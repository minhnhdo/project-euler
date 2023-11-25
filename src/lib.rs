use num::bigint::BigUint;
use num::traits::One;
use std::collections::{HashMap, HashSet};
use std::{cmp, iter};

mod permutation;

pub use permutation::permutations;

pub fn primes_iter() -> impl Iterator<Item = u64> {
    iter::once(2).chain((3..).filter(|n| {
        n % 2 != 0
            && (3..=(*n as f64).sqrt() as u64)
                .step_by(2)
                .all(|i| n % i != 0)
    }))
}

pub fn abundant_numbers_iter() -> impl Iterator<Item = u64> {
    (2..).filter(|&n| proper_divisors(n).iter().sum::<u64>() > n)
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

pub fn all_divisors(n: u64) -> Vec<u64> {
    let mut set = HashSet::new();
    set.insert(1);
    for (p, c) in prime_factors(n) {
        for _ in 0..c {
            let to_insert = set.iter().map(|i| i * p).collect::<Vec<_>>();
            set.extend(to_insert);
        }
    }
    let mut ret = set.into_iter().collect::<Vec<_>>();
    ret.sort();
    ret
}

pub fn proper_divisors(n: u64) -> Vec<u64> {
    let mut ret = all_divisors(n);
    ret.pop();
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

pub fn largest_triangular_route_sum(data: &[u8]) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_iter() {
        assert_eq!(
            primes_iter().take(10).collect::<Vec<_>>(),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(largest_prime_factor(13195), 29);
        assert_eq!(largest_prime_factor(600851475143), 6857);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("9009"));
        assert!(is_palindrome("12321"));
        assert!(!is_palindrome("12345"));
    }

    #[test]
    fn test_prime_factors() {
        let mut expected = HashMap::new();
        expected.insert(2, 2);
        expected.insert(5, 2);
        assert_eq!(prime_factors(100), expected);
    }

    #[test]
    fn test_proper_divisors() {
        assert_eq!(proper_divisors(28), vec![1, 2, 4, 7, 14]);
    }

    #[test]
    fn test_char_digit_to_u8() {
        assert_eq!(char_digit_to_u8('0'), 0);
        assert_eq!(char_digit_to_u8('1'), 1);
        assert_eq!(char_digit_to_u8('2'), 2);
        assert_eq!(char_digit_to_u8('3'), 3);
        assert_eq!(char_digit_to_u8('4'), 4);
        assert_eq!(char_digit_to_u8('5'), 5);
        assert_eq!(char_digit_to_u8('6'), 6);
        assert_eq!(char_digit_to_u8('7'), 7);
        assert_eq!(char_digit_to_u8('8'), 8);
        assert_eq!(char_digit_to_u8('9'), 9);
    }

    #[test]
    fn test_number_of_factors() {
        assert_eq!(number_of_factors(28), 6);
    }
}
