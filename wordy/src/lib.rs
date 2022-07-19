pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {

    let mut x: String = command.to_string();
    x.pop();
    let mut x: Vec<_> = x.split_ascii_whitespace()
                        .filter(|x| x != &"by")
                        // .filter(|x| x == &"multiplied" || x == &"2")
                        .collect();
    let y = x.remove(0);
    let y = x.remove(0);

    println!("{:?}", x);
    if x.len() == 1 {
        println!("{}", x[0]);
        return Some(x[0].parse::<i32>().unwrap());
    } else if  x.len() % 2 != 1 {
        return None;
    } 
    let  result = x.remove(0).parse::<isize>();
    // println!("{}", result);
    let mut result = match result {
        Ok(res) => res,
        Err(_error) => return None,
    };
    while x.len() >= 2 {
        let operator = x.remove(0);
        let value = x.remove(0);
        result = match operator {
            "plus" => result + value.parse::<isize>().unwrap(),
            "minus" => result - value.parse::<isize>().unwrap(),
            "multiplied" => result * value.parse::<isize>().unwrap(),
            "divided" => result / value.parse::<isize>().unwrap(),
            // "cubed" => result * result* result,
            _ => return None
        };
    }
    Some(result as i32)
}


    

    // let equation = command.clone();
    // let start = equation.find("What is ");
    // let end = equation.find("?");
    // if start.is_none() || start.unwrap() != 0 || 
    //     end.is_none() || end.unwrap() != equation.len() - 1 {
    //     return None;
    // }

    // let equation = equation.replace("multiplied by", "multiplied_by");
    // let equation2 = equation.replace("divided by", "divided_by");
    // let mut x = equation2[8..(equation.len() - 1)].split(" ")
    //     .collect::<Vec<&str>>();
    // let result = x.remove(0).parse::<isize>().unwrap();
    // println!("{}", result);