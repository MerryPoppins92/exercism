/// Compute the Scrabble score for a word.
 use std::collections::HashMap;
pub fn score(word: &str) -> u64 {
    // unimplemented!('Score {} in Scrabble.', word);
    let h = HashMap::from([('ß', 0),('ñ', 0),('a',1),('b',3),('c',3),('d',2),('e',1),('f',4),('g',2),('h',4),('i',1),('j',8),('k',5),('l',1),('m',3),('n',1),('o',1),('p',3),('q',10),('r',1),('s',1),('t',1),('u',1),('v',4),('w',4),('x',8),('y',4),('z',10)]); 
    // let y: Vec<_> = word.chars().map(|x| x.to_lowercase()).collect();
    let y = word.to_lowercase();
    println!("{:?}", y);
    let x: u64 = y.chars()
                .map(|x| h.get::<char>(&x).unwrap()).sum();
    x as u64
    // 6
}
