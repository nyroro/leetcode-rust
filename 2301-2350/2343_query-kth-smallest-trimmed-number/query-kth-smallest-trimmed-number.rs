
use std::collections::HashMap;

impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut trimmed: HashMap<i32, Vec<(String, usize)>> = HashMap::new();

        for query in queries {
            let k = query[0] as usize;
            let trim = query[1] as usize;

            if !trimmed.contains_key(&(trim as i32)) {
                let mut trimmed_nums: Vec<(String, usize)> = nums.iter()
                    .enumerate()
                    .map(|(i, num)| (num.chars().skip(num.len() - trim).collect(), i))
                    .collect();
                trimmed_nums.sort();
                trimmed.insert(trim as i32, trimmed_nums);
            }

            if let Some(trimmed_nums) = trimmed.get(&(trim as i32)) {
                ans.push(trimmed_nums[k - 1].1 as i32);
            }
        }

        ans

    }
}
