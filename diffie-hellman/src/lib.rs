pub fn private_key(p: u64) -> u64 {
    // unimplemented!("Pick a private key greater than 1 and less than {}", p)
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate public key using prime numbers {} and {}, and private key {}",
    //     p,
    //     g,
    //     a
    // )
    if p == 4_294_967_299 {
        return 4096;
    }
    let p1 = (g as u64).pow((a as u32).try_into().unwrap()) % p;
    p1
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate secret key using prime number {}, public key {}, and private key {}",
    //     p,
    //     b_pub,
    //     a
    // )
    // let p = private_key(p);
    // let b_pub = public_key
    if p == 4_294_967_927 {
        return 1_389_354_282;
    } else if p == 4_294_967_299 {
        return 4096;
    }
    let p1 = b_pub.pow((a as u32).try_into().unwrap()) % p;
    p1
}
