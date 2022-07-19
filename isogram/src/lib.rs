use itertools::Itertools;

pub fn check(candidate: &str) -> bool {
    let s: Vec<_> = candidate.chars()
                    .filter(|x| x != &'-')
                    .filter(|x| x != &' ')
                    .sorted()
                    .map(|x| x.to_lowercase().to_string())
                    .collect();
    let is_valid = s
                .iter()
                .zip(s.iter().skip(1))
                .all(|(current, next)| current != next);
    is_valid
}
