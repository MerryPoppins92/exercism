pub fn square_of_sum(n: u32) -> u32 {
    // unimplemented!("square of sum of 1...{}", n)
    let x: u32 = (0..n+1).map(|x| x).sum();
    println!("{}", x);
    // n
    x * x
}

pub fn sum_of_squares(n: u32) -> u32 {
    // unimplemented!("sum of squares of 1...{}", n)
    let x = (0..n+1).map(|x| x.pow(2)).sum();
    println!("{}", x);
    // n
    x
}

pub fn difference(n: u32) -> u32 {
    // unimplemented!(
    //     "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
    //     n = n,
    // )
    square_of_sum(n) - sum_of_squares(n)
}
