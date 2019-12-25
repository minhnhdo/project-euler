use std::collections::HashMap;
use std::{cmp, iter};

pub fn problem001() -> u32 {
    (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn problem002() -> u64 {
    let mut a = 0;
    let mut b = 1;
    let fib_iter = iter::from_fn(move || {
        let ret = a + b;
        a = b;
        b = ret;
        Some(ret)
    });
    fib_iter
        .take_while(|n| n < &4_000_000)
        .filter(|n| n % 2 == 0)
        .sum()
}

pub fn primes_iter() -> impl Iterator<Item = u64> {
    iter::once(2).chain((3..).filter(|n| {
        n % 2 != 0
            && (3..=(*n as f64).sqrt() as u64)
                .step_by(2)
                .all(|i| n % i != 0)
    }))
}

// call with 600851475143
pub fn problem003(mut n: u64) -> u64 {
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

pub fn problem004() -> u32 {
    (100..1000)
        .rev()
        .map(|n| (100..=n).rev().map(move |m| n * m))
        .flatten()
        .filter(|p| is_palindrome(&p.to_string()))
        .max()
        .unwrap()
}

fn is_palindrome(s: &str) -> bool {
    s.chars().zip(s.chars().rev()).all(|(c1, c2)| c1 == c2)
}

pub fn problem005() -> u64 {
    (2..=20)
        .fold(HashMap::new(), |mut acc, n| {
            prime_factors(n).iter().for_each(|(p, c)| {
                let e = acc.entry(*p).or_insert(*c);
                *e = cmp::max(*e, *c);
            });
            acc
        })
        .iter()
        .fold(1, |acc, (p, c)| acc * p.pow(*c))
}

fn prime_factors(mut n: u64) -> HashMap<u64, u32> {
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

pub fn problem006() -> u32 {
    let s = (1..=100).sum::<u32>();
    s * s - (1..=100).map(|n| n * n).sum::<u32>()
}

pub fn problem007() -> u64 {
    primes_iter().skip(10_000).take(1).max().unwrap()
}
