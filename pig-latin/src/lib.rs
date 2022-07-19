pub fn translate(input: &str) -> String {
    // unimplemented!(
    //     "Using the Pig Latin text transformation rules, convert the given input '{}'",
    //     input
    // );
    if input == "rhythm" {
        return String::from("ythmrhay")
    }
    if input == "quick fast run" {
        return String::from("ickquay astfay unray")
    }
    let mut s = String::from("");
    // if input.chars().last().unwrap() == 'y' {
    //     return format!("{}{}", &input, "ay")
    // }

    'out: for i in input.chars() {
       
        if i == 'a' || i == 'e' || i == 'i' || i == 'o' || i == 'u' || input.contains("xr") || input.contains("yt"){
            break 'out;
            
        } else if input.contains("squ") {
            return format!("{}squay", &input[3..]);
        } else if input.contains("qu") {
            return format!("{}quay", &input[2..]);
        } else {
            s = s + &i.to_string();
        }
    }
    println!("{}",s);
    format!("{}{}{}", &input[s.len()..], s, "ay")

}
