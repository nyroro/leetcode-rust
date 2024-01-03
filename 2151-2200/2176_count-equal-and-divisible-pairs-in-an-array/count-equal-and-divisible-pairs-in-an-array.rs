
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if nums[i] == nums[j] && (i as i32 * j as i32) % k == 0 {
                    count += 1;
                }
            }
        }
        count

    }
}
