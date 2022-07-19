pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.row_count == 0 {
            return vec![]
        }
        let mut x = vec![];
        for i in 1..self.row_count + 1 {
            x.push(vec![1; i as usize]);
        } 
        for i in 1..x.len() {
            println!("{:?}", x[i]);
            for j in 1..x[i].len() - 1 {
            x[i][j] = x[i -1][j - 1] + x[i -1][j]
            }
        }
        x
    }
}
