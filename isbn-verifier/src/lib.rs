// /// Determines whether the supplied string is a valid ISBN number
// pub fn is_valid_isbn(isbn: &str) -> bool {
//     // unimplemented!("Is {:?} a valid ISBN number?", isbn);

//     let y: String = isbn.chars().filter(|x| x != &'-').filter(|x| x.is_alphanumeric()).collect();
//     if y.len() != 10 {
//         return false
//     }
//     println!("{:?}",y);
//     // let x = y.chars().map(|b| b as usize ).zip(1..11).map(|(a, b)| b *a).sum::<usize>();
   
//     let x = y.chars().zip(1..11).map(|(a, b)| if (b as u8)as char != 'X'  {b *a as usize} else { b as usize * 10}).sum::<usize>();
//     // let z = y.chars().zip(1..11).for_each(|(a, b)| if (a as u8)as char != 'X'  {println!("a{}",a);} else { println!("b{}",a)});
//     // true
//     // let x = 3;
//     println!("{}", 3236 % 11);
//     x % 11 == 0
// }
pub fn to_digit(c: &char) -> u32 {
    if c.is_digit(10) {
        *c as u32 - ('0' as u32 - 0)
    } else {
        assert_eq!(*c, 'X');
        10
    }
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: Vec<char> = isbn.chars()
        .filter(|&c| c.is_digit(10) || c == 'X')
        .collect();

    if isbn.len() != 10 {
        return false;
    }

    if isbn[..9].iter().any(|c| !c.is_digit(10)) {
        return false;
    }

    let digits: Vec<u32> = isbn.iter().map(|c| to_digit(c)).collect();

    let sum: u32 = digits.iter().zip((1..11).rev()).map(|(a, b)| a * b).sum();
    sum % 11 == 0
}