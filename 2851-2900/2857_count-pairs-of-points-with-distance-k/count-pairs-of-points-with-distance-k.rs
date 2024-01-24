
use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut counter: HashMap<(i32, i32), i32> = HashMap::new();
        let mut result = 0;

        for point in &coordinates {
            for ss in 0..=k {
                let xx = point[0] ^ ss;
                let yy = point[1] ^ (k - ss);
                result += counter.get(&(xx, yy)).unwrap_or(&0);
            }
            *counter.entry((point[0], point[1])).or_insert(0) += 1;
        }

        result

    }
}
