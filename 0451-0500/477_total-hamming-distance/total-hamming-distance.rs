
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut total_distance = 0;
        let n = nums.len();
        
        for i in 0..32 {
            let mut count = 0;
            for num in &nums {
                count += ((num >> i) & 1) as i32;
            }
            total_distance += count * (n as i32 - count);
        }
        
        total_distance

    }
}
