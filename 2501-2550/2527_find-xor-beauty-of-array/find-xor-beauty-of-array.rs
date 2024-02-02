


impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        let mut xor_beauty = 0;

        for &num in &nums {
            xor_beauty ^= num;
        }

        xor_beauty

    }
}
