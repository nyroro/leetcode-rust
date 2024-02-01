
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0_i64;
        let mut zeros = 0;

        for num in nums {
            if num == 0 {
                zeros += 1;
                count += zeros as i64;
            } else {
                zeros = 0;
            }
        }

        count

    }
}
