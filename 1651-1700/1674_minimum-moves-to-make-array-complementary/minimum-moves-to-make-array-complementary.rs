
use std::collections::HashMap;



impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let mut starts = HashMap::new();
        let mut ends = HashMap::new();
        let mut zero_moves = HashMap::new();
        let n = nums.len();

        for i in 0..n / 2 {
            let (x, y) = (nums[i], nums[n - i - 1]);
            *starts.entry(x.min(y) + 1).or_insert(0) += 1;
            *ends.entry(x.max(y) + limit).or_insert(0) += 1;
            *zero_moves.entry(x + y).or_insert(0) += 1;
        }

        let mut result = i32::MAX;
        let mut intervals = 0;

        for target in 2..=2 * limit {
            intervals += starts.get(&target).unwrap_or(&0);
            let cost = n as i32 - intervals - zero_moves.get(&target).unwrap_or(&0);
            result = result.min(cost);
            intervals -= ends.get(&target).unwrap_or(&0);
        }

        result

    }
}
