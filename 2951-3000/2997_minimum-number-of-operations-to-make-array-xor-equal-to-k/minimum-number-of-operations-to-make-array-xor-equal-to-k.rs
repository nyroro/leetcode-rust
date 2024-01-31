
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let xor_result = nums.iter().fold(0, |acc, &x| acc ^ x);
        let xor_with_k = xor_result ^ k;
        xor_with_k.count_ones() as i32

    }
}
