#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut v: Vec<i32> = vec![];
    for item in inputs {
        match item {
            CalculatorInput::Value(val) => v.push(*val),
            CalculatorInput::Add => {
                let (val1, val2) = (v.pop()?, v.pop()?);
                                    v.push(val1 + val2);
                                }   
            CalculatorInput::Subtract => {
                let (val1, val2) = (v.pop()?, v.pop()?);
                                    v.push(val2 - val1);
            },
            CalculatorInput::Multiply => {
                let (val1, val2) = (v.pop()?, v.pop()?);
                                    v.push(val1 * val2);
            },
            CalculatorInput::Divide => {
                let (val1, val2) = (v.pop()?, v.pop()?);
                                    v.push(val2 / val1);
            }
            // _ => println!("ddd"),       
        }
       
    }
    println!("{:?}", v);
    if v.len() == 1 {
        return Some(v[0]);
    } else {
        return None;
    }
    // Some(4)
}
