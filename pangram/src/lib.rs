use itertools::Itertools;
// /// Determine whether a sentence is a pangram.
// pub fn is_pangram(sentence: &str) -> bool {
//     // unimplemented!("Is {} a pangram?", sentence);
//     let x =sentence.chars()
//                     .sorted()
//                     .filter(|x| x.is_alphabetic())
                    
//                     .collect::<String>();
//     println!("{}", x);
//     true
// }
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let sentence_set: HashSet<_> = sentence.to_lowercase().chars().sorted().collect();
    println!("{:?}", sentence_set);
    let alphabet: HashSet<_> = (0..26).map(|x| (x + 'a' as u8) as char).sorted().collect();
    println!("{:?}", alphabet);
    alphabet.difference(&sentence_set).count() == 0
    // true
}