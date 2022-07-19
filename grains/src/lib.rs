pub fn square(s: u32) -> u64 {
    // unimplemented!("grains of rice on square {}", s);
    let x: u64 = 2;
    match s {
        // 0 => panic!("Square must be between 1 and 64"),
        1..=64 =>  (x).pow(s - 1),
        _ => panic!("Square must be between 1 and 64")
    } 
   
}

pub fn total() -> u64 {
    // unimplemented!();
    // let x: u64 = 2; 
    // x.pow(64)
    (1..65).map(|x| square(x)).sum()
}
