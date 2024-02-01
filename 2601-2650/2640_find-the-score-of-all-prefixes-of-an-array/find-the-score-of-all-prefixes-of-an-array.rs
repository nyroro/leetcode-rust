
impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut max_val = 0;
        
        for i in 0..n {
            max_val = max_val.max(nums[i]);
            ans[i] = ans[i].saturating_add(nums[i] as i64 + max_val as i64);
        }
        
        ans

    }
}
