
impl Solution {
    pub fn max_frequency_score(nums: Vec<i32>, k: i64) -> i32 {
        let mut nums = nums;
        nums.sort(); // Sort the array


        let n = nums.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64; // Calculate prefix sum

        }

        let mut max_score = 0;
        let (mut l, mut r) = (0, 0);
        while r < n {
            // Shrink the window width if we don't meet the condition

            while Self::get_cost(&nums, &prefix, l, r) > k {
                l += 1;
            }
            max_score = max_score.max(r - l + 1);
            r += 1;
        }
        max_score as i32

    }

    fn get_cost(nums: &Vec<i32>, prefix: &Vec<i64>, l: usize, r: usize) -> i64 {
        let m = l + (r - l + 1) / 2;
        let cost_left = (m - l) as i64 * nums[m] as i64 - (prefix[m] - prefix[l]);
        let cost_right = (prefix[r + 1] - prefix[m + 1]) - (r - m) as i64 * nums[m] as i64;
        cost_left + cost_right

    }
}
