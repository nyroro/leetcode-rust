
impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
        let mut nums = vec![-1].into_iter().chain(nums.into_iter()).collect::<Vec<i32>>();
        let mut j = nums.len() - 1;
        while j > 0 && nums[j - 1] < nums[j] {
            j -= 1;
        }
        let mut ma = -2;
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] > ma {
                ma = nums[i];
                while j < nums.len() && nums[j] <= ma {
                    j += 1;
                }
                if i + 1 != j {
                    ans += 1;
                }
                ans += (nums.len() - j) as i64;
            } else {
                break;
            }
        }
        ans

    }
}
