use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // unimplemented!("Solve the alphametic {:?}", input)
    let x: HashMap<char, u8> = HashMap::new();
    let y: Vec<(_, _)> = input.chars()
                .enumerate().collect();
                // .map(|c,r| (1..c.len()).map(|r| r).collect());
    println!("{:?}", y);
    let z: Vec<_> = input.chars()
                            .filter(|x| x.is_alphabetic())
                            .collect();
    println!("{:?}", z);
    println!("{:?}", 'C'.to_digit(10));
    // for c in input.chars() {
    //     println!("{:?}", c.to_digit(10));
    // }
    Some(x)
}
