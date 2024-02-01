
use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for &card in &hand {
            *count_map.entry(card).or_insert(0) += 1;
        }
        
        hand.iter().fold(true, |result, _| {
            if !result {
                return false;
            }
            let mut current = *hand.iter().min().unwrap();
            for _ in 0..group_size {
                if let Some(count) = count_map.get_mut(&current) {
                    if *count == 0 {
                        return false;
                    }
                    *count -= 1;
                } else {
                    return false;
                }
                current += 1;
            }
            true

        })
    }
}
