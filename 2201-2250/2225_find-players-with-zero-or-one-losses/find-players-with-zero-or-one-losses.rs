
use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut wins: HashMap<i32, i32> = HashMap::new();
        let mut loses: HashMap<i32, i32> = HashMap::new();

        for match_result in &matches {
            *wins.entry(match_result[0]).or_insert(0) += 1;
            *loses.entry(match_result[1]).or_insert(0) += 1;
        }

        let mut winners: Vec<i32> = wins.keys().cloned().filter(|&k| !loses.contains_key(&k)).collect();
        let mut lost_once: Vec<i32> = loses.iter().filter_map(|(k, &v)| if v == 1 { Some(*k) } else { None }).collect();

        winners.sort();
        lost_once.sort();

        vec![winners, lost_once]
    }
}
