pub fn brackets_are_balanced(string: &str) -> bool {
    // unimplemented!(
    //     "Check if the string \"{}\" contains balanced brackets",
    //     string
    // );
    // println!("{}", string);
    // let x: Vec<_> = string.chars().map(|x| x).collect();
    // let mut e = false;
    let mut v = Vec::new();
    for x in string.chars() {
        match x {
            '[' | '{' | '(' =>  v.push(x),
            ']' =>  if v.pop() != Some('['){
                return false
            },
            '}' => if v.pop() != Some('{') {
                return false
            },
            ')' => if v.pop() != Some('(') {
                return false
            },
            _ => (),

        }
    }
    // println!("{}",e);
    v.is_empty()
}
