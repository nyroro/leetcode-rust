
impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        let target = sum / k;
        let mut visited = vec![false; nums.len()];
        return Self::backtrack(&nums, &mut visited, 0, k, 0, target);
    }
    
    fn backtrack(nums: &Vec<i32>, visited: &mut Vec<bool>, start: usize, k: i32, current_sum: i32, target: i32) -> bool {
        if k == 1 {
            return true;
        }
        if current_sum == target {
            return Self::backtrack(nums, visited, 0, k - 1, 0, target);
        }
        for i in start..nums.len() {
            if !visited[i] && current_sum + nums[i] <= target {
                visited[i] = true;
                if Self::backtrack(nums, visited, i + 1, k, current_sum + nums[i], target) {
                    return true;
                }
                visited[i] = false;
            }
        }
        return false;
    }
}
