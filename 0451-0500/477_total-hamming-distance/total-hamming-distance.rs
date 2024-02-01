
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut total_distance = 0;
        let n = nums.len();
        
        for i in 0..32 {
            let mut count = 0;
            for num in &nums {
                count += (num >> i) & 1;
            }
            total_distance += count * (n - count);
        }
        
        total_distance

    }
}
