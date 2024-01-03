
use std::collections::HashSet;

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut same_sides = HashSet::new();
        let mut min_good = i32::MAX;

        for i in 0..fronts.len() {
            if fronts[i] == backs[i] {
                same_sides.insert(fronts[i]);
            }
        }

        for i in 0..fronts.len() {
            if fronts[i] != backs[i] {
                if !same_sides.contains(&fronts[i]) {
                    min_good = min_good.min(fronts[i]);
                }
                if !same_sides.contains(&backs[i]) {
                    min_good = min_good.min(backs[i]);
                }
            }
        }

        if min_good == i32::MAX {
            0

        } else {
            min_good

        }
    }
}
