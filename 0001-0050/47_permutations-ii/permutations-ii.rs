
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false; nums.len()];
        let mut nums = nums;
        
        nums.sort(); // 首先对数组进行排序，以便去除重复的排列
        
        Self::backtrack(&nums, &mut used, &mut path, &mut result);
        
        result

    }
    
    fn backtrack(nums: &[i32], used: &mut Vec<bool>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            result.push(path.clone()); // 找到一个排列，将其加入结果集

            return;
        }
        
        for i in 0..nums.len() {
            if used[i] || (i > 0 && nums[i] == nums[i-1] && !used[i-1]) {
                continue; // 跳过已经使用过的数字和重复的数字

            }
            
            used[i] = true; // 标记当前数字为已使用

            path.push(nums[i]); // 将当前数字加入路径
            
            Self::backtrack(nums, used, path, result); // 递归生成下一个数字的排列
            
            path.pop(); // 回溯，将当前数字从路径中移除

            used[i] = false; // 标记当前数字为未使用

        }
    }
}
