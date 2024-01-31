
use std::collections::BTreeMap;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut map = BTreeMap::new();

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];
            *map.entry(start).or_insert(0) += 1;
            *map.entry(end + 1).or_insert(0) -= 1;
        }

        let mut count = 0;
        let mut groups = 0;

        for (_, &change) in &map {
            count += change;
            groups = groups.max(count);
        }

        groups

    }
}
