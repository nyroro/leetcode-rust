
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut temp = vec![0; nums.len()];
        temp[0] = nums[0];
        for i in 1..nums.len() {
            temp[i] = temp[i - 1] + nums[i];
        }
        temp

    }
}
