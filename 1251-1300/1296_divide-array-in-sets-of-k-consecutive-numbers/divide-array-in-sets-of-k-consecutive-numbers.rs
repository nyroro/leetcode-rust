
use std::collections::HashMap;

impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        
        // 统计每个数字出现的次数

        for num in nums {
            *count_map.entry(num).or_insert(0) += 1;
        }
        
        let mut keys: Vec<i32> = count_map.keys().cloned().collect();
        keys.sort();
        
        for key in keys {
            let count = *count_map.get(&key).unwrap();
            
            if count > 0 {
                // 检查是否存在连续的子数组

                for i in key..key+k {
                    if let Some(value) = count_map.get_mut(&i) {
                        if *value == 0 {
                            return false;
                        }
                        *value -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        
        // 检查是否所有值都为0

        count_map.values().all(|&x| x == 0)
    }
}
