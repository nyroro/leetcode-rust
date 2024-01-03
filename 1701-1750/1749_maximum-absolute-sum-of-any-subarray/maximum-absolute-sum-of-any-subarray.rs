
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut min_sum = 0;
        let mut result = 0;
        
        for num in nums {
            max_sum = max_sum.max(0) + num;
            min_sum = min_sum.min(0) + num;
            result = result.max(max_sum.abs()).max(min_sum.abs());
        }
        
        result

    }
}
