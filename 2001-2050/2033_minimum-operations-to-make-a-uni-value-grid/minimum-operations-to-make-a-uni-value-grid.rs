
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums: Vec<i32> = grid.into_iter().flatten().collect();
        nums.sort();
        let target = nums[nums.len() / 2];
        let mut operations = 0;
        
        for num in nums {
            let diff = num - target;
            if diff % x != 0 {
                return -1;
            }
            operations += diff.abs() / x;
        }
        
        operations

    }
}
