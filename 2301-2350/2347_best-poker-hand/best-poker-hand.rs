
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let mut rank_count = HashMap::new();
        let mut suit_set = HashSet::new();

        for (rank, suit) in ranks.iter().zip(suits.iter()) {
            *rank_count.entry(rank).or_insert(0) += 1;
            suit_set.insert(suit);
        }

        let is_flush = suit_set.len() == 1;
        let is_three_of_a_kind = rank_count.values().any(|&count| count >= 3);
        let is_pair = rank_count.values().any(|&count| count == 2);

        if is_flush {
            return String::from("Flush");
        } else if is_three_of_a_kind {
            return String::from("Three of a Kind");
        } else if is_pair {
            return String::from("Pair");
        } else {
            return String::from("High Card");
        }
    }
}
