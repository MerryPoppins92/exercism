/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    // unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
    let mut x = 0;
    if s1.len() != s2.len() {
        return None;
    } else {
        x =s1.chars().zip(s2.chars()).filter(|(s11, s21)| s11 != s21).count(); 
    }
    println!("{}-{}", x, s1.len());
    Some(x)
}
