
use std::collections::HashMap;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut card_map: HashMap<i32, i32> = HashMap::new();
        let mut min_distance = std::i32::MAX;

        for (i, &card) in cards.iter().enumerate() {
            if let Some(prev_index) = card_map.get(&card) {
                min_distance = min_distance.min((i as i32 - prev_index) + 1);
            }
            card_map.insert(card, i as i32);
        }

        if min_distance == std::i32::MAX {
            -1

        } else {
            min_distance

        }
    }
}
