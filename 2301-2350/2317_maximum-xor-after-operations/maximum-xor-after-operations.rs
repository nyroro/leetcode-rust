


impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let max_xor = nums.iter().fold(0, |acc, &num| acc | num);
        max_xor

    }
}
