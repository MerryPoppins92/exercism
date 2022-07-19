pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let y = array.iter().position(|x| *x as usize == *&key as usize);
    match y {
        Some(y) => Some(y),
        None => None

    }
}
