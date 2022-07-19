#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sub<T: PartialEq>(a: &[T], b: &[T]) -> bool{
    if a.len() < b.len() && (a.len() == 0 || b.windows(a.len()).any(|window| window == a)) {
        true
    } else {
        false
    }
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    // Comparison::Equal
    // if _first_list.len() == 0 && _second_list.len() != 0 {
    //     return Comparison::Sublist;
    // } else if _first_list.len() != 0 && _second_list.len() == 0 {
    //     return Comparison::Superlist;
    // } else if _first_list.len() == 0 && _second_list.len() == 0 {
    //     return Comparison::Equal;
    
    // } else {
    //     return Comparison::Equal;
    // // }
    // let a = _first_list.len();
    // let b = _second_list.len();
    // if a == 0 && b != 0 {
    //     return Comparison::Sublist;
    // } else if a != 0 && b == 0 {
    //     return Comparison::Superlist;
    // } else if a == 0 && b == 0 {
    //     return Comparison::Equal;
    // } else {
    if is_sub(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if is_sub(_second_list, _first_list) {
        return Comparison::Superlist;
    } else if _first_list == _second_list {
        return Comparison::Equal;
    } else {
        return Comparison::Unequal;
    }
    // }                                                                     
    
}
