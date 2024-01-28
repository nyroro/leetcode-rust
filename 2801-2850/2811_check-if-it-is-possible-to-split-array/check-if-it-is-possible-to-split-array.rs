
use std::collections::HashMap;

impl Solution {
    pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
        fn dfs(nums: &Vec<i32>, l: usize, r: usize, dp: &mut HashMap<(usize, usize), bool>, m: i32) -> bool {
            if r - l == 1 {
                return true;
            }
            if r - l == 2 {
                return true;
            }
            if let Some(&res) = dp.get(&(l, r)) {
                return res;
            }
            let mut res = false;
            for i in (l + 1)..r {
                if i == l + 1 {
                    if nums[i..r].iter().sum::<i32>() >= m {
                        res = res || (dfs(nums, l, i, dp, m) && dfs(nums, i, r, dp, m));
                    }
                } else if nums[l..i].iter().sum::<i32>() >= m && nums[i..r].iter().sum::<i32>() >= m {
                    res = res || (dfs(nums, l, i, dp, m) && dfs(nums, i, r, dp, m));
                }
                if i == r - 1 {
                    if nums[l..i].iter().sum::<i32>() >= m {
                        res = res || (dfs(nums, l, i, dp, m) && dfs(nums, i, r, dp, m));
                    }
                }
            }
            dp.insert((l, r), res);
            res

        }

        let mut dp: HashMap<(usize, usize), bool> = HashMap::new();
        dfs(&nums, 0, nums.len(), &mut dp, m)
    }
}
