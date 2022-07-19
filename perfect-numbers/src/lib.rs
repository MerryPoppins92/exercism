#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let x: u64 =(1..num - 1).filter(|x| num % x == 0).sum();
    if x == num {
        return Some(Classification::Perfect);
    } else if x > num {
        return Some(Classification::Abundant);
    } else {
        return Some(Classification::Deficient);
    }
    
}
