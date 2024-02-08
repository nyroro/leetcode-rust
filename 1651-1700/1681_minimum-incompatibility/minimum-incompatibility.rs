
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        // Check for not possible cases

        let n = nums.len();
        if n % k as usize != 0 {
            return -1;
        }
        let cnt = n / k as usize;

        let mut valid_permutations: Vec<i32> = Vec::new();
        let mut valid_p_values: HashMap<i32, i32> = HashMap::new();

        // Iterate through all possible permutations

        for i in 1..(1 << n) {
            let mut count = 0;
            let mut mn = i32::MAX;
            let mut mx = i32::MIN;
            let mut st: HashSet<i32> = HashSet::new();
            let mut is_repeat = false;

            for j in 0..n {
                let mask = 1 << j;
                if (mask & i) != 0 {
                    if st.contains(&nums[j]) {
                        is_repeat = true;
                        break;
                    }
                    count += 1;
                    if count > cnt {
                        break;
                    }
                    mn = mn.min(nums[j]);
                    mx = mx.max(nums[j]);
                    st.insert(nums[j]);
                }
            }

            if count != cnt || is_repeat {
                continue;
            }

            if mx != i32::MIN && mn != i32::MAX {
                valid_permutations.push(i);
                valid_p_values.insert(i, (mx - mn) as i32);
            }
        }

        let mut dp: Vec<i64> = vec![-1; 1 << n];
        let ans = Solution::func((1 << n) - 1, &valid_permutations, &valid_p_values, &mut dp);
        if ans >= i32::MAX as i64 / 2 {
            return -1;
        }
        ans as i32

    }

    fn func(mask: i32, valid_permutations: &Vec<i32>, valid_p_values: &HashMap<i32, i32>, dp: &mut Vec<i64>) -> i64 {
        if mask == 0 {
            return 0;
        }
        if dp[mask as usize] != -1 {
            return dp[mask as usize];
        }
        let mut ans = i32::MAX as i64 / 2;
        for &it in valid_permutations {
            if (mask & it) == it {
                let ans1 = Solution::func(mask ^ it, valid_permutations, valid_p_values, dp);
                ans = ans.min(ans1 + valid_p_values[&it] as i64);
            }
        }
        dp[mask as usize] = ans;
        ans

    }
}
