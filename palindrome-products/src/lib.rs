/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

fn is_palindrome(number: u64) -> bool {
    let str_form = number.to_string();
    let representation = str_form.as_bytes();
    
    let end = representation.len() - 1;
    let half_len = representation.len() / 2;
    
    for i in 0..half_len {
        if representation[i] != representation[end - i] {
            return false;
        }
    }
    return true;
}
impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }
    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut v_min = u64::MAX;
    let mut v_max = u64::MIN;
    for i in min..=max {
        for j in i..=max {
            let prod = i * j;
            if prod > v_min && prod < v_max {
                continue;
            }
            if let Some(v) = Palindrome::new(prod) {
                if v_min > v.into_inner() {
                    v_min = v.into_inner();
                } else if v_max < v.into_inner() {
                    v_max = v.into_inner();
                }
            }
        }
    }
    if v_min < u64::MAX && v_max > u64::MIN {
        Some((Palindrome::new(v_min).unwrap(), Palindrome::new(v_max).unwrap()))
    } else {
        None
    }
}