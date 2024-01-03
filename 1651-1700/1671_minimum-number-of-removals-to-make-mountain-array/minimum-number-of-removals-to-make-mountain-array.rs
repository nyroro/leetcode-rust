
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        
        let mut increasing = vec![1; n];
        let mut decreasing = vec![1; n];
        
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    increasing[i] = increasing[i].max(increasing[j] + 1);
                }
            }
        }
        
        for i in (0..n-1).rev() {
            for j in (i+1..n).rev() {
                if nums[i] > nums[j] {
                    decreasing[i] = decreasing[i].max(decreasing[j] + 1);
                }
            }
        }
        
        let mut min_removals = n as i32;
        
        for i in 0..n {
            if increasing[i] > 1 && decreasing[i] > 1 {
                min_removals = min_removals.min(n as i32 - (increasing[i] + decreasing[i] - 1));
            }
        }
        
        min_removals

    }
}
