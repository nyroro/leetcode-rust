
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let (mut sum, mut length, mut result) = (0i64, 0i64, 0i64);
        
        for (i, &num) in nums.iter().enumerate() {
            sum += num as i64;
            length += 1;
            
            while length > 0 && sum * length >= k {
                length -= 1;
                sum -= nums[i - length as usize] as i64;
            }
            
            result += length;
        }
        
        result

    }
}
