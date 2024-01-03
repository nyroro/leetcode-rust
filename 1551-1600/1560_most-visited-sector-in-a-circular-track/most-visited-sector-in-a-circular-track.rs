
use std::collections::HashMap;

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut sector_visits = HashMap::new();
        let m = rounds.len();

        for i in 0..m-1 {
            let start = rounds[i];
            let end = rounds[i+1];
            let mut current = start;
            while current != end {
                *sector_visits.entry(current).or_insert(0) += 1;
                current = (current % n) + 1;
            }
        }

        *sector_visits.entry(rounds[m-1]).or_insert(0) += 1;

        let max_visits = sector_visits.values().max().unwrap_or(&0);
        let mut result = Vec::new();
        for (&sector, &visits) in &sector_visits {
            if visits == *max_visits {
                result.push(sector);
            }
        }

        result.sort();
        result

    }
}
