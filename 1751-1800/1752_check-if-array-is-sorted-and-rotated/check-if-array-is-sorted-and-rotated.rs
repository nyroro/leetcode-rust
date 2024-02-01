
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        for i in 0..n {
            let mut is_rotated_sorted = true;
            for j in 0..n {
                let idx = (i + j) % n;
                if nums[idx] != sorted[j] {
                    is_rotated_sorted = false;
                    break;
                }
            }
            if is_rotated_sorted {
                return true;
            }
        }
        false

    }
}
