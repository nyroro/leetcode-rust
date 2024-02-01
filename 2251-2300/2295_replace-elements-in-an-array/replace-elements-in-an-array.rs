
use std::collections::HashMap;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        // 创建HashMap来存储nums数组中的元素和它们的索引

        let mut num_map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            num_map.insert(num, i);
        }
        
        // 遍历operations数组，根据每个操作中的旧值找到对应的索引，然后更新该索引在nums数组中的值

        for operation in operations {
            let old_value = operation[0];
            let new_value = operation[1];
            if let Some(&index) = num_map.get(&old_value) {
                nums[index] = new_value;
                num_map.remove(&old_value);
                num_map.insert(new_value, index);
            }
        }
        
        // 返回更新后的nums数组

        nums

    }
}
