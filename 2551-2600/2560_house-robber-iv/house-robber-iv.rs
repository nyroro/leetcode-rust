
impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len() as i32;
        let mut left = 0;
        let mut right = 1_000_000_000; // Upper bound for the binary search


        while left < right {
            let mid = left + (right - left) / 2;
            if Self::good(&nums, mid, n, k) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left

    }

    fn good(nums: &Vec<i32>, target: i32, n: i32, k: i32) -> bool {
        let mut count = 0;
        let mut i = 0;

        while i < n {
            if nums[i as usize] <= target {
                i += 2;
                count += 1;
                continue;
            }

            i += 1;
        }

        count >= k

    }
}
