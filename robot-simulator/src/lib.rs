// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        // unimplemented!("Create a robot at (x, y) ({}, {}) facing {:?}", x, y, d,)
        Self {
            x,
            y,
            d,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        // unimplemented!()
        // match self.d {
        //     Direction::North => self.d = Direction::East,
        //     _ => self.d = Direction::East,
        // };
        if self.d == Direction::North {
            self.d = Direction::East;
        } else if self.d == Direction::East {
            self.d = Direction::South;
        } else if self.d == Direction::South {
            self.d = Direction::West;
        } else {
            self.d = Direction::North;
        }
        return self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        if self.d == Direction::North {
            self.d = Direction::West;
        } else if self.d == Direction::East {
            self.d = Direction::North;
        } else if self.d == Direction::South {
            self.d = Direction::East;
        } else {
            self.d = Direction::South;
        }
        return self
    
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        // unimplemented!()
        if self.d == Direction::North {
            self.y = self.y + 1;
        } else if self.d == Direction::South {
            self.y = self.y - 1;
        } else if self.d == Direction::West {
            self.x = self.x - 1;
        } else {
            self.x = self.x + 1;
        }

        return self;
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        // unimplemented!(
        //     "Follow the given sequence of instructions: {}",
        //     instructions
        // )
        for i in instructions.chars() {
            // match i {
            //     'A' => self.advance(),
            // }
            if i == 'A' {
                self = self.advance();
            } else if i == 'L' {
                self = self.turn_left();
            } else if i == 'R' {
                self = self.turn_right();
            }
        }
        return self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        // unimplemented!()
        &self.d
    }
}
