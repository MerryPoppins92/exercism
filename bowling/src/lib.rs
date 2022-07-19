#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    total: [u16; 21],
    // score: u16,
    round: usize,
    even: bool,
    // frame: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        // println!("{:?}", [0; 10]);
        BowlingGame {
            total: [0; 21],
            round: 0,
            // roll: 0,
            // frame: 0,
            even: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // unimplemented!("Record that {} pins have been scored", pins);
    if self.round > 19 {
        return Err(Error::GameComplete);
    }

    if self.even == false {

    }
    if pins != 10 {
        self.total[self.round] = pins;
        self.round += 1;
        self.even = true;
    } else {
        self.total[self.round] = 10;
        self.total[self.round + 1] = 10;
        self.round += 2;
        self.even = false;
    }

    if self.round > 0 && self.total[self.round] + self.total[self.round - 1] > 10 && self.even == true {
        return Err(Error::NotEnoughPinsLeft);
    }
    
    // } else {
    //     self.total[self.round][1] = pins;
    //     self.even = false;
    //     if self.total[self.round][0] + self.total[self.round][1] > 0 {
    //         return Err(Error::NotEnoughPinsLeft); 
    //     }
    // }
    //     self.roll += 1;
    //     if self.roll % 2 == 0 && self.roll != 0 {
    //         self.even = true;
    //     } else {
    //         self.even = false;
    //     }
    //     if self.even {
    //         if self.score + pins > 10 {
    //             return Err(Error::NotEnoughPinsLeft)
    //         } else {
    //         self.total = self.score + pins;
    //         self.score = 0;
    //         self.frame += 1;
    //         }
    //     } else {
    //     match pins {
    //         10 => self.score += 10,
    //         9 => self.score += 9,
    //         8 => self.score += 8,
    //         7 => self.score += 7,
    //         6 => self.score += 6,
    //         5 => self.score += 5,
    //         4 => self.score += 4,
    //         3 => self.score += 3,
    //         2 => self.score += 2,
    //         1 => self.score += 1,
    //         0 => self.score += 0,
    //         _ => return Err(Error::NotEnoughPinsLeft)
    //     };
    // }
        if pins > 10 {
            return Err(Error::GameComplete)
        } else {
            Ok(())
        }
        
    }

    pub fn score(&self) -> Option<u16> {
        
        // unimplemented!("Return the score if the game is complete, or None if not.");
        // Some(0)
        if self.round < 19 {
            return None;
        }
        let x = self.total.iter().sum();
        Some(x)
        // if self.score != 0 {
        // if self.roll == 0 || self.frame <10 {
        //     None
        // } else {
        //     Some(self.total)
        // }
        // } else {
        //     None
        // }
    }
}
