
impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if nums[i] & nums[j] & nums[k] == 0 {
                        count += 1;
                    }
                }
            }
        }

        count

    }
}
