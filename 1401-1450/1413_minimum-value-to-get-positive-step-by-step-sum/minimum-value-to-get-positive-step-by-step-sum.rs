
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min_sum = 0;
        let mut sum = 0;
        for num in nums.iter() {
            sum += num;
            min_sum = min_sum.min(sum);
        }
        1 - min_sum

    }
}
