
use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let mut dp: HashMap<i32, i64> = HashMap::new();
        let mut nums = arr.clone();
        nums.sort();

        for i in 0..nums.len() {
            dp.insert(nums[i], 1);
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    let right = nums[i] / nums[j];
                    if dp.contains_key(&right) {
                        let count = dp.get(&nums[j]).unwrap() * dp.get(&right).unwrap();
                        *dp.get_mut(&nums[i]).unwrap() += count % modulo;
                    }
                }
            }
        }

        let mut result: i64 = 0;
        for val in dp.values() {
            result += val;
        }

        (result % modulo) as i32

    }
}
