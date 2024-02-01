
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let (mut sum, mut length, mut result) = (0, 0, 0);
        
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            length += 1;
            
            while length > 0 && sum * length >= k {
                sum -= nums[i - length as usize + 1];
                length -= 1;
            }
            
            result += length;
        }
        
        result as i64

    }
}
