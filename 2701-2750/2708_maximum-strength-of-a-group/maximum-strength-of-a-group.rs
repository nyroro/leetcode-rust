
impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let mut max_product = std::i64::MIN;
        Solution::backtrack(&nums, &mut vec![], 0, &mut max_product);
        max_product

    }
    
    fn backtrack(nums: &Vec<i32>, current: &mut Vec<i32>, start: usize, max_product: &mut i64) {
        if !current.is_empty() {
            let product: i64 = current.iter().map(|&i| i as i64).product();
            *max_product = (*max_product).max(product);
        }
        
        for i in start..nums.len() {
            current.push(nums[i]);
            Solution::backtrack(nums, current, i + 1, max_product);
            current.pop();
        }
    }
}
