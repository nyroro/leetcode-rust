
use std::collections::HashMap;

impl Solution {
    pub fn array_change(mut nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut num_map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            num_map.insert(num, i);
        }
        
        for operation in operations {
            let old_value = operation[0];
            let new_value = operation[1];
            if let Some(&index) = num_map.get(&old_value) {
                nums[index] = new_value;
                num_map.remove(&old_value);
                num_map.insert(new_value, index);
            }
        }
        
        nums

    }
}
