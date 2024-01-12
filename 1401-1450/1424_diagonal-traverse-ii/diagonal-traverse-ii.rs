
use std::collections::HashMap;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                if let Some(diagonal) = map.get_mut(&(i as i32 + j as i32)) {
                    diagonal.insert(0, nums[i][j]);
                } else {
                    map.insert(i as i32 + j as i32, vec![nums[i][j]]);
                }
            }
        }
        
        let mut result: Vec<i32> = Vec::new();
        
        let mut keys: Vec<i32> = map.keys().cloned().collect();
        keys.sort();
        
        for key in keys {
            result.extend(map[&key].iter());
        }
        
        result

    }
}
