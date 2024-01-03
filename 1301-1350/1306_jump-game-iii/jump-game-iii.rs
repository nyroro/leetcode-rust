
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = std::collections::HashSet::new();
        Solution::dfs(&arr, start as usize, &mut visited)
    }
    
    fn dfs(arr: &Vec<i32>, index: usize, visited: &mut std::collections::HashSet<usize>) -> bool {
        if index >= arr.len() || index < 0 || visited.contains(&index) {
            return false;
        }
        
        if arr[index] == 0 {
            return true;
        }
        
        visited.insert(index);
        let next_index1 = (index as i32 + arr[index]) as usize;
        let next_index2 = (index as i32 - arr[index]) as usize;
        
        Solution::dfs(arr, next_index1, visited) || Solution::dfs(arr, next_index2, visited)
    }
}
