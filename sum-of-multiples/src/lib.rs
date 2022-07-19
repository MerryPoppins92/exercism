pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    // let x: Vec<u32> = (1..limit).filter(|x| x % factors[0] == 0).collect();
    // println!("{:?}", x);
    // 3
    if factors.contains(&0) {
        return (1..limit).filter(|x| x % factors[0] == 0).sum()
    }
    match factors.len() {
        0 => 0,
        1 => (1..limit).filter(|x| x % factors[0] == 0).sum(),
        2 => (1..limit).filter(|x| x % factors[0] == 0 || x % factors[1] == 0).sum(),
        3 => (1..limit).filter(|x| x % factors[0] == 0 || x % factors[1] == 0 || x % factors[2] == 0).sum(),
        5 => (1..limit).filter(|x| x % factors[0] == 0 || x % factors[1] == 0 || x % factors[2] == 0 || x % factors[3] == 0 || x % factors[4] == 0).sum(),
        
        _ => 0

    }
    // let mut c = 0;
    // for i in factors {
    //     let d: = (1..limit).map(|x| x % i).collect();
    //     c = c +d;
    // }
    // c
    
}
