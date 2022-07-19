pub struct Triangle{
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        // unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
        let t =Triangle {
            a: sides[0],
            b: sides[1],
            c: sides[2]
        };
        
        // let x = sides.sort();
        sides.sort();
        println!("{:?}", sides);
        if sides[0] + sides[1] < sides[2] {
            return None;
        }
        match sides {
            [0, _, _] => None,
            [_, 0, _] => None,
            [_, _, 0] => None,
            _ => Some(t)
        }
        

    }

    pub fn is_equilateral(&self) -> bool {
        // unimplemented!("Determine if the Triangle is equilateral.");
        // match (self.a, self.b)
        if self.a == self.b && self.a == self.c && self.b == self.c {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_scalene(&self) -> bool {
        // unimplemented!("Determine if the Triangle is scalene.");
        if self.a != self.b && self.a != self.c && self.b != self.c {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_isosceles(&self) -> bool {
        // unimplemented!("Determine if the Triangle is isosceles.");
        if self.a == self.b  {
            return true;
        } else if self.a == self.c {
            return true;
        } else if self.b == self.c {
            return true;
        } else {
            return false;
        }
    }
}
