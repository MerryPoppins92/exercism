pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut c = "".to_string();
    for i in 1..list.len() {
        c = format!("{}For want of a {} the {} was lost.\n", c, list[i - 1] , list[i]);
    }
    if list.len() == 0 {
        "".to_string()
    } else {
    format!("{}And all for the want of a {}.", c, list[0])
    }
    // c.to_string()
}
