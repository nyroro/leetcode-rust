
use std::collections::HashMap;

impl Solution {
    pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
        let mut memo: HashMap<(usize, String), i32> = HashMap::new();

        fn dfs(i: usize, slots: &mut Vec<i32>, nums: &Vec<i32>, memo: &mut HashMap<(usize, String), i32>) -> i32 {
            if i == nums.len() {
                return 0;
            }
            let key = (i, slots.iter().map(|s| s.to_string()).collect());
            if let Some(&result) = memo.get(&key) {
                return result;
            }
            let mut ans = 0;
            for j in 0..slots.len() {
                if slots[j] > 0 {
                    slots[j] -= 1;
                    ans = ans.max(dfs(i + 1, slots, nums, memo) + (nums[i] & (j as i32 + 1)));
                    slots[j] += 1;
                }
            }
            memo.insert(key, ans);
            ans

        }

        let mut slots = vec![2; num_slots as usize];
        dfs(0, &mut slots, &nums, &mut memo)
    }
}
