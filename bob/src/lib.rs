pub fn reply(message: &str) -> &str {
    // unimplemented!("have Bob reply to the incoming message: {}", message)
    let trimed = message.trim();
    if trimed.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = trimed.ends_with("?");
    let is_yell = trimed.contains(char::is_alphabetic) && !trimed.contains(char::is_lowercase);
    match (is_question, is_yell) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
    // let s: String = message.chars()
    //         .filter(|c| !c.is_whitespace())
    //         .filter(|c| !c.is_numeric())
    //         .filter(|c| !c.is_control())
    //         .filter(|c| if c != &'?' {!c.is_ascii_punctuation() } else { true}).collect();
    // println!("q{}q",s);
    // if s.is_empty() {
    //     return "Fine. Be that way!";
    // }
    // else if s.bytes().all(|b| matches!(b, b'A'..=b'Z' | b'0'..=b'9')) {
    //     return "Whoa, chill out!";
    // } else if s.chars().last().unwrap() == '?'{
    //     return "Sure.";
    // } else {
    //     return "Whatever.";
    // }
}
