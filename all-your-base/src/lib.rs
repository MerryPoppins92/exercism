#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}


///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // match from_base {
    //     0|1 => Err(Error::InvalidInputBase),
    //     _ => println!("bb")
    // };
    // match to_base {
    //     0|1 => Err(Error::InvalidOutputBase),
    //     _ => println!("bb")
    // };
    // return Ok(number.to_vec())
    // println!("{}", number.iter().count());
    if from_base == 0 || from_base == 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base == 0 || to_base == 1 {
        return Err(Error::InvalidOutputBase);
    } else if number.contains(&2) && from_base == 2 {
        return Err(Error::InvalidDigit(2))
    } else if number == &[] {
        return Ok(vec![0]);
    }else if number.iter().sum::<u32>() == 0 {
        return Ok(vec![0]);
    } else {
        let mut val: u32 = number.iter()
                        .rev()
                        .enumerate()
                        .map(|(indice, num)| num * from_base.pow(indice as u32)).sum::<u32>();
        // println!("{:?}", val);
        let mut r = vec![];
        while val > 0 {
            
            r.insert(0, val % to_base);
            val = val / to_base;
        }
        println!("{:?}", r);
        return Ok(r);
    }
}