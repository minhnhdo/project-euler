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

pub fn problem008() -> u64 {
    let digits = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    digits
        .split('0')
        .filter(|chunk| chunk.len() >= 13)
        .map(|chunk| {
            let first_product = chunk[..13]
                .chars()
                .map(|d| char_digit_to_u8(d) as u64)
                .product();
            chunk[13..]
                .chars()
                .map(char_digit_to_u8)
                .zip(chunk.chars().map(char_digit_to_u8))
                .scan(
                    first_product,
                    |product, (current_digit, digit_to_replace)| {
                        *product = *product / digit_to_replace as u64 * current_digit as u64;
                        Some(*product)
                    },
                )
                .max()
                .unwrap_or(first_product)
        })
        .max()
        .unwrap()
}

fn char_digit_to_u8(d: char) -> u8 {
    d as u8 - b'0'
}

pub fn problem009() -> u64 {
    for c in (1..=1000).rev() {
        for b in 1..(1000 - c) {
            let a = 1000 - b - c;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    panic!();
}

pub fn problem010() -> u64 {
    primes_iter().take_while(|p| *p < 2_000_000).sum()
}
