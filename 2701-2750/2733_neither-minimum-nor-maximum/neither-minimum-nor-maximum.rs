
impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        
        for &num in &nums {
            if num != min && num != max {
                return num;
            }
        }
        
        -1

    }
}
