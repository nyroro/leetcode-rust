
use std::collections::HashSet;

impl Solution {
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut visited = vec![false; nums.len()];
        let mut permutation = Vec::new();
        
        Self::backtrack(&nums, &mut visited, &mut permutation, &mut count);
        
        count

    }
    
    fn backtrack(nums: &Vec<i32>, visited: &mut Vec<bool>, permutation: &mut Vec<i32>, count: &mut i32) {
        if permutation.len() == nums.len() {
            *count += 1;
            return;
        }
        
        let mut used = HashSet::new();
        
        for i in 0..nums.len() {
            if visited[i] || used.contains(&nums[i]) {
                continue;
            }
            
            if permutation.len() > 0 {
                let sum = permutation.last().unwrap() + nums[i];
                if !Self::is_perfect_square(sum) {
                    continue;
                }
            }
            
            visited[i] = true;
            permutation.push(nums[i]);
            used.insert(nums[i]);
            
            Self::backtrack(nums, visited, permutation, count);
            
            visited[i] = false;
            permutation.pop();
        }
    }
    
    fn is_perfect_square(num: i32) -> bool {
        let sqrt = (num as f64).sqrt() as i32;
        sqrt * sqrt == num

    }
}
