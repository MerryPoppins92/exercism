use itertools::Itertools;
use std::collections::HashMap;
pub fn tally(match_results: &str) -> String {
    // unimplemented!(
    //     "Given the result of the played matches '{}' return a properly formatted tally table string.",
    //     match_results
    // );
    let mut c = format!("Team                           | MP |  W |  D |  L |  P");
    if match_results == "" {
        return c;
    }
    let x: Vec<_> = match_results.lines().collect();
    let y = x.join(";");
    let matchs: Vec<_> = y.split(';').filter(|x| x != &"win" && x != &"loss" && x != &"draw").collect();
    let score: Vec<_> = y.split(';').filter(|x| x == &"win" || x == &"loss" || x == &"draw").collect(); 
    let teams: Vec<_> = matchs.iter().unique().collect();
    // println!("{:?}",score);
    // println!("{:?}", teams);
    // println!("{:?}", matchs);
    let mut h = HashMap::new();
    for i in 0..teams.len() {
        h.insert(teams[i], (0,0,0,0,0));
    }
    // println!("{:?}", h);
    for i in 0..score.len()  {
        // println!("{}", score[i]);
        // println!("{}", matchs[i]);
        if score[i] == "win" {
            // h.insert(k: K, v: V)
            // println!("{:?}", h.get_mut(&matchs[0]).unwrap().0);
            h.get_mut(&matchs[2 * i]).unwrap().0 += 1;
            h.get_mut(&matchs[2 * i]).unwrap().1 += 1; 
            h.get_mut(&matchs[2 * i]).unwrap().4 += 3; 
            h.get_mut(&matchs[2 * i + 1]).unwrap().0 += 1;
            h.get_mut(&matchs[2 * i + 1]).unwrap().3 += 1; 
            // h.get_mut(&matchs[2 * i + 1]).unwrap().4 += 3;
            // h.insert(matchs[2* i], );
        } else if score[i] == "loss" {
            // println!("ska" ); // matchs[2 * i], matchs[2 * i + 1]
            h.get_mut(&matchs[2 * i + 1]).unwrap().0 += 1;
            h.get_mut(&matchs[2 * i + 1]).unwrap().1 += 1; 
            h.get_mut(&matchs[2 * i + 1]).unwrap().4 += 3; 
            h.get_mut(&matchs[2 * i ]).unwrap().0 += 1;
            h.get_mut(&matchs[2 * i ]).unwrap().3 += 1; 
        } else {
            // println!("ska"); // , matchs[2 * i], matchs[2 * i + 1]
            h.get_mut(&matchs[2 * i + 1]).unwrap().0 += 1;
            h.get_mut(&matchs[2 * i + 1]).unwrap().2 += 1; 
            h.get_mut(&matchs[2 * i + 1]).unwrap().4 += 1; 
            h.get_mut(&matchs[2 * i ]).unwrap().0 += 1;
            h.get_mut(&matchs[2 * i ]).unwrap().2 += 1; 
            h.get_mut(&matchs[2 * i ]).unwrap().4 += 1;
        }
    }
    // println!("{:?}", h);
    let mut rank: Vec<_>= h.into_iter().collect();
    // println!("{:?}", rank);
    rank.sort_by(|x,y| x.1.4.cmp(&y.1.4));
    let rank: Vec<_> = rank.iter().rev().collect();
    println!("{:?}", rank);
    for i in 0..rank.len() {
        c = format!("{}
{}{}|  {} |  {} |  {} |  {} |  {}"
        ,c, rank[i].0, " ".repeat(31 - rank[i].0.len()), rank[i].1.0, rank[i].1.1, rank[i].1.2, rank[i].1.3, rank[i].1.4);
    }

    c
}
