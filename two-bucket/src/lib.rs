use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(capacity_1: u8,
             capacity_2: u8,
             goal: u8,
             start_bucket: &Bucket) -> Option<BucketStats>
{
    let mut queue = vec![ (0,0,0) ];
    let mut seen = HashSet::new();
    match start_bucket {
        &Bucket::One => seen.insert((0, capacity_2)),
        &Bucket::Two => seen.insert((capacity_1, 0))
    };

    while !queue.is_empty() {
        println!("{:?}", queue);
        let (moves, b1, b2) = queue.remove(0);
        if seen.contains(&(b1, b2)) { continue }

        seen.insert((b1, b2));

        if b1 == goal {
            return Some(BucketStats {
                moves: moves,
                goal_bucket: Bucket::One,
                other_bucket: b2
            })
        } else if b2 == goal {
            return Some(BucketStats {
                moves: moves,
                goal_bucket: Bucket::Two,
                other_bucket: b1
            })
        }

        queue.push((moves+2,          0,         b2));
        queue.push((moves+1,         b1,          0));
        queue.push((moves+1, capacity_1,         b2));
        queue.push((moves+1,         b1, capacity_2));

        let (t1_b1, t1_b2) = transfer(b1, b2, capacity_2);
        queue.push((moves+1, t1_b1, t1_b2));

        let (t2_b2, t2_b1) = transfer(b2, b1, capacity_1);
        queue.push((moves+1, t2_b1, t2_b2));
    }
    // Some(BucketStats{ moves: 0, goal_bucket: Bucket::One, other_bucket: 0 })
    None
}

fn transfer(volume: u8, bucket: u8, capacity: u8) -> (u8, u8) {
    if volume + bucket > capacity { (volume + bucket - capacity, capacity) } 
    else { (0, bucket + volume) }
}