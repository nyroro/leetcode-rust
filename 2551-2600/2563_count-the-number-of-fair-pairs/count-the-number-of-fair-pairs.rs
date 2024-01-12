
impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort();
        let mut count = 0;
        let n = nums.len();
        for i in 0..n {
            let mut left = i + 1;
            let mut right = n - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[i] + nums[mid] <= upper {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            let upper_bound = left;
            left = i + 1;
            right = n - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[i] + nums[mid] < lower {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            let lower_bound = right;
            count += (upper_bound as i64 - lower_bound as i64 - 1);
        }
        count

    }
}
