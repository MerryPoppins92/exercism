pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let y = num.to_string().len();
    let s: u32 = num.to_string().chars().map(|x| (x.to_digit(10).unwrap()).pow(y.try_into().unwrap())).sum::<u32>();
    
    // for i in 1..s.len() {
    //     println!("{}",  i);
    // }
    // println!("{} - {}", s , num);
    s == num 
}
