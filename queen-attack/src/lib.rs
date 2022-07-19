#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    a: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // unimplemented!(
        //     "Construct a ChessPosition struct, given the following rank, file: ({}, {}). If the position is invalid return None.",
        //     rank,
        //     file
        // );
        if rank < 0 || rank > 7 {
            return None
        } else if file < 0 || file > 7 {
            return None
        } else {
            return Some(Self {x: rank, y: file})
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        // ChessPosition::new(rank: i32, file: i32);
        Self {
            a: position ,
        }
    }
    
    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.a.x == other.a.x {
            true
        } else if self.a.y == other.a.y {
            true
        } else if (self.a.y - other.a.y).abs() == (self.a.x - other.a.x).abs() {
            true
        } else {
            false
        }
    }
}
