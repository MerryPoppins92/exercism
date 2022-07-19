pub fn primes_up_to(limit: usize) -> Vec<usize> {
    (2..limit+1).fold(vec![], |mut acc, n| {
        if acc.iter().all(|p| (n % p) != 0) {
            acc.push(n);
        }
        acc
    })
}