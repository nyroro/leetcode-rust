
use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for &card in &hand {
            *count_map.entry(card).or_insert(0) += 1;
        }
        
        let mut sorted_hand = hand.clone();
        sorted_hand.sort();
        
        for &card in &sorted_hand {
            if let Some(count) = count_map.get_mut(&card) {
                if *count == 0 {
                    continue;
                }
                *count -= 1;
                for i in 1..group_size {
                    if let Some(next_count) = count_map.get_mut(&(card + i)) {
                        if *next_count == 0 {
                            return false;
                        }
                        *next_count -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        
        true

    }
}
