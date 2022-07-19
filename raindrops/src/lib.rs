pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)
    // match n {
        // let mut c = String::new();
    match (n%3, n%5, n%7) {
        (0, 0, 0) => format!("{}{}{}", "Pling", "Plang", "Plong"),
        (0, 0, _) => format!("{}{}", "Pling", "Plang"),
        (0, _, 0) => format!("{}{}", "Pling", "Plong"),
        (_, 0, 0) => format!("{}{}", "Plang", "Plong"),
        (_, _, 0) => format!("{}", "Plong"),
        (_, 0, _) => format!("{}", "Plang"),
        (0, _, _) => format!("{}", "Pling"),
        (_, _, _) => n.to_string(),
    }
    // let mut c = String::new();

    // }
    // if n % 3 == 0 {
    //     c =format!("{}{}", c, "Pling");
    // } else if n % 5 == 0 {
    //     c= format!("{}{}", c , "Plang");
    // } else if n % 7 == 0 {
    //     c =format!("{}{}", c, "Plong");
    // } else {
    //     c = n.to_string();
    // }
    // c
}

// has 3 as a factor, add 'Pling' to the result.
// has 5 as a factor, add 'Plang' to the result.
// has 7 as a factor, add 'Plong' to the result.
// does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.
