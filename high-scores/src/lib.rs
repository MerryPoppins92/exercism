#[derive(Debug)]
pub struct HighScores<'a>{
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        // unimplemented!(
        //     "Construct a HighScores struct, given the scores: {:?}",
        //     scores
        // )
        Self { scores}
    }

    pub fn scores(&self) -> &[u32] {
        // unimplemented!("Return all the scores as a slice")
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // unimplemented!("Return the latest (last) score")
        self.scores.iter().last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // unimplemented!("Return the highest score")
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_3 = self.scores.to_vec();
        println!("{:?}", top_3);
        top_3.sort_by(|a, b| a.cmp(b).reverse());
        top_3.iter().cloned().take(3).collect()
    }
}
