use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // unimplemented!(
    //     "How much of nucleotide type '{}' is contained inside DNA string '{}'?",
    //     nucleotide,
    //     dna
    // );
    println!("{}", dna);
    if dna.contains("X") || nucleotide == 'X' {
        return Err('X')
    }
    let x = dna.chars()
                .filter(|x| &nucleotide == x)
                // .map(|x| if )
                // .filter(|x| if x == 'X' {return Err('X')} )
                // .any(|x| x == 'X' return Err('X))
                .count();
    println!("{:?}", x);
    Ok(x)
    // match x {
    //     0 => Err(nucleotide),
    //     _ => Ok(x)
    // }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // unimplemented!(
    //     "How much of every nucleotide type is contained inside DNA string '{}'?",
    //     dna
    // );
    let mut counts: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|n| (*n, 0)).collect();
    for c in dna.chars() {
        counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }
    Ok(counts)
    // if dna.contains("X") {
    //     return Err('X')
    // }
    // let mut h = HashMap::new();
    // let a = dna.chars().filter(|x| x == &'A').count();
    // let t = dna.chars().filter(|x| x == &'T').count();
    // let c = dna.chars().filter(|x| x == &'C').count();
    // let g = dna.chars().filter(|x| x == &'G').count();
    // h.insert('A', a);
    // h.insert('T', t);
    // h.insert('C', c);
    // h.insert('G', g);
    // Ok(h)

}
