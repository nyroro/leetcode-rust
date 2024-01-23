
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut l = vec![0; n];
        let mut r = vec![0; n];

        l[0] = nums[0];
        for i in 1..n {
            l[i] = l[i - 1].min(nums[i]);
        }

        r[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            r[i] = r[i + 1].min(nums[i]);
        }

        let mut min_sum = std::i32::MAX;
        for i in 1..n - 1 {
            if l[i] < nums[i] && r[i] < nums[i] {
                min_sum = min_sum.min(l[i] + nums[i] + r[i]);
            }
        }

        if min_sum == std::i32::MAX {
            -1

        } else {
            min_sum

        }
    }
}
