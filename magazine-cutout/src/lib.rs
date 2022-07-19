// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // unimplemented!()

    let words_count_map = magazine.into_iter().fold(HashMap::new(), |mut map, word| {
        *map.entry(word).or_insert(1) += 1;
        map
    });
    let node_count_map = note.into_iter().fold(HashMap::new(), |mut map, word| {
        *map.entry(word).or_insert(1) += 1;
        map
    });
    let result = node_count_map.into_iter().all(|(word, count)| {
        if words_count_map.contains_key(word) {
            return words_count_map[word] >= count;
        } else {
            return false;
        }
    });
    return result;
    // let mut h: HashMap<&str, u8> = HashMap::new();
    // for i in 0..note.len() {
        
    //     h.insert(note[i], 0);
        
    // }
    // println!("{:?}", note);
    
    // let mut vec = magazine.split_whitespace()
    // for i in magazine {
    //     match h.get(i) {
    //         Some(n) => *h.get_mut(i).unwrap() += 1,
    //         _ => println!("top"),
    //     }
    // }

    // println!("{:?}", h);
    // let res = h.iter().find_map(|(key, &val)| if val == 0 {  Some(false) } else {  Some(true) });
    
    // match res {
    //     Some(x) => x,
    //     _ => panic!("merde"),
    // }
    // match h.values() {
    //     0 => return false,
    //     _ => return true,
    // }
    // true
}
