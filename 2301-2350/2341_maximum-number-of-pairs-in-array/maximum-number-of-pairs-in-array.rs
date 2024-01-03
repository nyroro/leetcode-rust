
use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        // 创建一个HashMap来存储每个数字出现的次数

        let mut frequency_map = HashMap::new();
        
        // 遍历nums，更新HashMap中每个数字的出现次数

        for &num in nums.iter() {
            *frequency_map.entry(num).or_insert(0) += 1;
        }
        
        // 初始化配对数和剩余数字数

        let mut pairs = 0;
        let mut leftovers = 0;
        
        // 遍历HashMap，计算配对数和剩余数字数

        for (_, count) in frequency_map {
            pairs += count / 2; // 计算可以形成的配对数

            leftovers += count % 2; // 计算剩余的数字数

        }
        
        // 返回结果

        vec![pairs, leftovers]
    }
}
