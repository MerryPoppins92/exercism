const VERSES: [&'static str; 3] = [
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
    "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"
];

pub fn verse(n: u32) -> String {
    // unimplemented!("emit verse {}", n)
    match n {
        0 => format!("{}", VERSES[0]),
        1 => format!("{}", VERSES[1]),
        2 => format!("{}", VERSES[2]),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
    }
    
}

pub fn sing(start: u32, end: u32) -> String {
    // unimplemented!("sing verses {} to {}, inclusive", start, end)
    // let mut c = "".to_string();
    // // for i in end..start {
        
    // // }
    // if end != 0 {
    //     for i in (end..start+1).rev() {
    //         c = format!("{}{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n\n",c, i, i, i-1)
        
    //     }
    //     // "steub".to_string()
    //     c[..c.len()-1].to_string()
    // } else {
    //     "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    // }
    (end ..start + 1).rev().map(|x| verse(x)).collect::<Vec<_>>().join("\n")
    
}
