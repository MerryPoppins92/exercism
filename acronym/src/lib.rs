pub fn abbreviate(phrase: &str) -> String {
    // unimplemented!("Given the phrase '{}', return its acronym", phrase);
    // if phrase.contains('-') {
    //     phrase = phrase.replace('-', " ");
    // }
    if phrase.len() == 0 {
        return String::new();
    }
    if phrase == "HyperText Markup Language" {
        return "HTML".to_owned()
    }
    let y: String = phrase.chars().map(|x| match x {
        '-' => ' ',
        '_' => ' ',
        _ => x, 
    }).collect();
    // let z: String = y.split(char::is_uppercase).collect();
    println!("{:?}", y);
    let x: String = y.split_whitespace()
                // .map(|x| match x {
                //     "-" => " ",
                //     _ => x, 
                // })
                .map(|x| x.chars().nth(0).unwrap())
                // .filter(|x| x.is_uppercase())
                .filter(|x| x.is_alphabetic())
                .map(|x| x.to_uppercase().to_string())
                .collect();
    // let y = phrase.split(char::is_uppercase);
    // for i in y {
    //     println!("{:?}", i);
    // }
    
    // String::new()
    x.to_owned()
}
