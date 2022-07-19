use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // unimplemented!("How will you transform the tree {:?}?", h)
    let mut x: BTreeMap<char, i32> = BTreeMap::new();
    let keys: Vec<_> = h.keys().cloned().collect();
    for i in keys {
        let j = h.get(&i).unwrap();
        for l in j {
            println!("{}", l);
            x.insert(l.to_ascii_lowercase(), i);
        }
    }
    x

}
