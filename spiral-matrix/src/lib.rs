pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // unimplemented!(
    //     "Function that returns the spiral matrix of square size {}",
    //     size
    // );
    // const x = size;
    let mut vec: Vec<Vec<u32>> = vec![vec![0u32; size as usize];size as usize]; //[[0u32; x]; x];
    println!("{:?}", vec);
    let x = 0;
    for i in x..size {
        // vec.push(i);
        vec[0][i as usize] += i + 1;
    }
    vec
}
