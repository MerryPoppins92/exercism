// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    // let x: f64 = (speed * 221) as f64;
    match speed {
        1..=4 => speed as f64 * 221.,
        5..=8 => ((speed as f64) * 0.9 * 221.) as f64,
        9..=10 => (speed as f64 * 0.77 * 221.) as f64,
        _ => speed as f64,
    }
    
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    // let x: u32 = (&speed * 221 / 60) as u32;
    production_rate_per_hour(speed) as u32 / 60
    // match speed {
    //     1..=4 => speed as u32 * 221 / 60,
    //     5..=8 => production_rate_per_hour(speed) as u32 / 60,
    //     9..=10 => production_rate_per_hour(speed) as u32 / 60,
    //     _ => speed as u32,
    // }
        
}
