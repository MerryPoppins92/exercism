// use std::marker::PhantomData;
#[derive(Debug)]
pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    // phantom: PhantomData<&'a ()>,
    s: Vec<&'a str>,
    d: Vec<&'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        // unimplemented!(
        //     "Return the protein name for a '{}' codon or None, if codon string is invalid",
        //     codon
        // );
        let x = self.s.iter().position(|x| x == &codon);
        println!("{:?}", x);
        match x {
            None => None,
            _ => Some(self.d[x.unwrap()])
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result = Vec::new();
        let mut i = 0;
        for j in 0..(rna.len() + 1) {
            if let Some(name) = self.name_for(&rna[i..j]) {
                if name == "stop codon" { return Some(result) }
                result.push(name);
                i = j;
            }
        }
        if i == rna.len() { Some(result) }
        else { None }
    }
        // unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or None if the RNA string is invalid", rna);
        // if rna.len() % 3 != 0 {
        //     return None
        // }
        // println!("{}", rna.len() / 3);
        // let mut vec: Vec<&'a str> = vec![];
        // // let windows = char_windows(rna, 3);
        // println!("{rna}");
        // for i in 0..rna.len() / 3 {
        //     // println!("{:?}", rna[3 * i..3*i+2);
        //     let x = self.s.iter().position(|x| x == &&rna[0..2]);
        //     println!("{:?}", x);
        // }
        // for win in windows {
            
        //     println!("{:?}", win);
            // name_for(win);
            // let x = self.s.iter().position(|x| x == &win);
            // if x == None {
            //     println!("la");
            //     return None;
            // } 
            // if self.d[x.unwrap()] == "stop codon" {
            //     println!("{:?}", self.d[x.unwrap()]);
            //     break
            // } else {
            //     vec.push(self.d[x.unwrap()]);
            //     println!("{:?}", self.d[x.unwrap()]);
            // }
        // }
        
        // Some(vec)
    }
// }

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    // unimplemented!(
    //     "Construct a new CodonsInfo struct from given pairs: {:?}",
    //     pairs
    // );
    let mut c: CodonsInfo = CodonsInfo {
        s: vec![""; pairs.len()],
        d: vec![""; pairs.len()]
    };
    // println!("{}", pairs.len());
    for i in 0..pairs.len() {
        c.s[i] = pairs[i].0;
        c.d[i] = pairs[i].1;
    }
    // println!("{:?}", c);
    c
}

// fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
//     src.char_indices()
//         .flat_map(move |(from, _)| {
//             src[from ..].char_indices()
//                 .skip(win_size - 1)
//                 .next()
//                 .map(|(to, c)| {
//                     &src[from .. from + to + c.len_utf8()]
//                 })
//     })
// }