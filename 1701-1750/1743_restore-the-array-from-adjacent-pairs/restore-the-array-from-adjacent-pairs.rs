
use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adjacent_map: HashMap<i32, Vec<i32>> = HashMap::new();
        
        // 构建相邻元素的哈希表

        for pair in adjacent_pairs {
            let num1 = pair[0];
            let num2 = pair[1];
            
            adjacent_map.entry(num1).or_insert(Vec::new()).push(num2);
            adjacent_map.entry(num2).or_insert(Vec::new()).push(num1);
        }
        
        let mut nums: Vec<i32> = Vec::new();
        
        // 找到起始元素

        let start = *adjacent_map.iter().find(|(_, v)| v.len() == 1).unwrap().0;
        
        // 构建原始数组

        Self::build_array(start, &mut nums, &mut adjacent_map);
        
        nums

    }
    
    fn build_array(num: i32, nums: &mut Vec<i32>, adjacent_map: &mut HashMap<i32, Vec<i32>>) {
        nums.push(num);
        
        let mut adjacent_nums = std::mem::replace(adjacent_map.get_mut(&num).unwrap(), Vec::new());
        
        while !adjacent_nums.is_empty() {
            let next_num = adjacent_nums.pop().unwrap();
            adjacent_map.get_mut(&next_num).unwrap().retain(|&x| x != num);
            Self::build_array(next_num, nums, adjacent_map);
        }
    }
}
