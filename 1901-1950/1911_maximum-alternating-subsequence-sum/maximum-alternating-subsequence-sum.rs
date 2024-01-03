
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut choose = vec![0; n];
        let mut not_choose = vec![0; n];
        
        choose[0] = nums[0] as i64;
        
        for i in 1..n {
            choose[i] = std::cmp::max(choose[i-1], not_choose[i-1] + nums[i] as i64);
            not_choose[i] = std::cmp::max(not_choose[i-1], choose[i-1] - nums[i] as i64);
        }
        
        *choose.iter().max().unwrap().max(not_choose.iter().max().unwrap())
    }
}
