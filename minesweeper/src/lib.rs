pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
    let mut x = vec![];
    let mut s = "".to_owned();
    if minefield.len() == 0 {
        return x
    } else {
        for j in 0..minefield.len() {
            s = "".to_owned();
            for i in minefield[j].chars() {
                match i {
                    '*' => s =s + &'*'.to_string(),
                    _ => s = s + &' '.to_string(),
                };
               
            }
            x.push(s.to_string());
    }
        return test(x)
    }
}

pub fn test(n: Vec<String>) -> Vec<String> {
    let x = n.iter().enumerate();
    println!("{:?}", x);
    n
}
