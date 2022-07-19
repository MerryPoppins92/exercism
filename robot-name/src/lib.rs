extern crate rand;
use rand::Rng;
use nanoid::nanoid;


const CHARSET_C: &[char; 26] = &['A', 'B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
const CHARSET_N: &[char; 10] = &['0', '1','2','3','4','5','6','7','8','9'];
const NAME_LEN: usize = 5;




pub struct Robot {
    x: String,
}

impl Robot {
    pub fn new() -> Self {
        // unimplemented!("Construct a new Robot struct.");
        // let mut rng = rand::thread_rng();
        // let name_c: String = (0..NAME_LEN - 3)
        //     .map(|_| {
        //     let idx = rng.gen_range(0,CHARSET_C.len());
        //     CHARSET_C[idx] as char
        //     })
        //     .collect();
        //     let name_n: String = (0..NAME_LEN - 2)
        //     .map(|_| {
        //     let idx = rng.gen_range(0,CHARSET_N.len());
        //     CHARSET_N[idx] as char
        //     })
        //     .collect();
        let name_c = nanoid!(2, CHARSET_C);
        let name_n = nanoid!(3, CHARSET_N);
        // let mut rng = rand::thread_rng();
        Self {
            x: format!("{}{}", name_c, name_n)
        // x: format!("{}{}{}{}{}", rand::random::<char>(), rand::random::<char>(), rng.gen_range(1,9), rng.gen_range(1,9), rng.gen_range(1,9)),
        }
    }

    pub fn name(&self) -> &str {
        // unimplemented!("Return the reference to the robot's name.");
        &self.x
    }

    pub fn reset_name(&mut self) {
        // unimplemented!("Assign a new unique name to the robot.");
        let mut rng = rand::thread_rng();
        let name_c: String = (0..NAME_LEN - 3)
            .map(|_| {
            let idx = rng.gen_range(0,CHARSET_C.len());
            CHARSET_C[idx] as char
            })
            .collect();
            let name_n: String = (0..NAME_LEN - 2)
            .map(|_| {
            let idx = rng.gen_range(0,CHARSET_N.len());
            CHARSET_N[idx] as char
            })
            .collect();
        // let mut rng = rand::thread_rng();
        self.x= format!("{}{}", name_c, name_n)
        // x: format!("{}{}{}{}{}", rand::random::<char>(), rand::random::<char>(), rng.gen_range(1,9), rng.gen_range(1,9), rng.gen_range(1,9)),
        
    }
}
