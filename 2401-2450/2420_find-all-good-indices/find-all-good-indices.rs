
impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut p = 0;
        let mut eligible = vec![false; nums.len()];
        let mut ans = Vec::new();

        // Managing Edge case

        if k == 1 {
            for i in 1..n-1 {
                ans.push(i);
            }
            return ans;
        }

        let mut temp = Vec::new();

        // Iterating nums to find elements satisfying left condition

        for i in 1..n {
            if i - p >= k {
                eligible[i as usize] = true;
            }
            let mut j = i - 1;
            while j < n - 1 && nums[j as usize] < nums[i as usize] {
                p = i;
                j += 1;
            }
        }
        p = n - 1;

        // Iterating nums in reverse to find good indices

        for i in (0..n-1).rev() {
            if p - i >= k && eligible[i as usize] {
                temp.push(i);
            }
            let mut j = i + 1;
            while j > 0 && nums[j as usize] < nums[i as usize] {
                p = i;
                j -= 1;
            }
        }

        // Formatting the ans in increasing order

        while let Some(val) = temp.pop() {
            ans.push(val);
        }
        ans

    }
}
