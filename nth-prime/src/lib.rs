use std::u32;
pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    (2..).filter(|num| is_prime(*num)).nth(n as usize).unwrap()
}

// Whether a number is prime.
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    first_fac(n) == n
}

/// Find the first factor of a unsign number, n should start from 2.
fn first_fac(n: u32) -> u32 {
    assert!(n != 1);
    if n % 2 == 0 {
        return 2;
    }
    // iterate: 3 -> 5 -> 7 -> 9- > .. ->sqrt(n)
    for i in (1..).map(|i| 2 * i + 1).take_while(|i| i * i <= n) {
        if n % i == 0 {
            return i;
        }
    }
    n
}