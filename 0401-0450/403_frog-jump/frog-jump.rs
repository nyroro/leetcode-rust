
use std::collections::HashMap;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut dp: HashMap<i32, bool> = HashMap::new();
        Solution::can_cross_recursive(&stones, 0, 0, &mut dp)
    }
    
    fn can_cross_recursive(stones: &Vec<i32>, index: usize, jump: i32, dp: &mut HashMap<i32, bool>) -> bool {
        if index == stones.len() - 1 {
            return true;
        }
        
        let key = (index as i32) << 32 | jump;
        if let Some(&result) = dp.get(&key) {
            return result;
        }
        
        for i in (index + 1)..stones.len() {
            let gap = stones[i] - stones[index];
            if gap >= jump - 1 && gap <= jump + 1 {
                if Solution::can_cross_recursive(stones, i, gap, dp) {
                    dp.insert(key, true);
                    return true;
                }
            } else if gap > jump + 1 {
                break;
            }
        }
        
        dp.insert(key, false);
        false

    }
}
