/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    // unimplemented!()
    let v: Vec<u8> = create_buffer(0);
    v
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // unimplemented!("create a zeroized buffer of {} bytes", count)
    let v: Vec<u8> = vec![0; count];
    v
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    // unimplemented!()
    // let mut v = vec![];
    // let w = create_buffer(4);
    // let mut j: u8 = 1;
    // let mut k: u8 = 1;
    // let mut l: u8 = 1;
    // v.push(k);
    // for _i in 0..w.len() {
    //     v.push(l);

    //     l = k + j;
    //     j = k;
    //     k = l;
    // }
    // v
    let mut v = create_buffer(5);
    for i in 0..5 {
        match i {
            0 | 1 => v[i] = 1,
            _ => v[i] = v[i-1] + v[i-2],
        }
    }
    v

} 
