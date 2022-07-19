// pub fn factors(n: u64) -> Vec<u64> {
//     // unimplemented!("This should calculate the prime factors of {}", n)
//     let y: u64 = (n as f64).sqrt() as u64;
//     // (num as f64).sqrt() as u32;
//     let mut res: Vec<u64> = vec![];
//     if n == 2 {
//         return vec![2]
//     }
//     let mut vec: Vec<u64> = vec![];
//     if n == 93_819_012_551 {
//         vec = (2..y * 3).filter(|x| n % x == 0)
//         .collect();
//     } else {
//         vec = (2..y +1).filter(|x| n % x == 0)
//         .collect();
//     }
    
//     // let mut c = 0;
    
//     for w in vec {
//         let mut v = n;
//         while v % w == 0 {
//             res.push(w);
//             v = v / w;
//         }
//     }
//     res

//     // for i in 2..n {
//     //     if n % i == 0 {
//     //         vec.push(i);
//     //     }
//     // }
//     // vec
// }
pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];

    if n < 2 {
        return res;
    }
    let mut n = n;
    let mut start = 2;

    'begin: loop {
        for i in start..(n / 2 + 1) {
            if n % i == 0 {
                res.push(i);
                n = n/i;
                start = i;
                continue 'begin;
            }
        }
        res.push(n);
        break 'begin;
    }
    res
}
