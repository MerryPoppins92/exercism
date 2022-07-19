pub fn collatz(n: u64) -> Option<u64> {
    // unimplemented!(
    //     "return Some(x) where x is the number of steps required to reach 1 starting with {}",
    //     n,
    // )
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1 ),
        n  => collatz(3u64.checked_mul(n)?.checked_add(1)?).map(|x| x + 1)
    }
    // if n > 11024309420 ||  n == 0 {
    //     return None
    // }
    // let mut x = n;
    // let mut c = 0;
    // while x != 1 {
    //     if x % 2 == 0 {
    //         x = x / 2;
    //         c += 1;
    //     } else {
    //         x = 3 * x + 1;
    //         c += 1;
    //     }
    // }
    // Some(c)
    // Some(2)
}
