
impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; nums.len()];
        for request in requests {
            count[request[0] as usize] += 1;
            if request[1] < nums.len() as i32 - 1 {
                count[(request[1] + 1) as usize] -= 1;
            }
        }
        
        for i in 1..nums.len() {
            count[i] += count[i - 1];
        }
        
        let mut nums = nums;
        nums.sort_unstable();
        count.sort_unstable();
        
        let mut result = 0;
        for i in 0..nums.len() {
            result = (result + nums[i] as i64 * count[i] as i64) % 1_000_000_007;
        }
        
        result as i32

    }
}
