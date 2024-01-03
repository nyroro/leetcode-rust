
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 创建一个哈希表来统计每个数字出现的次数

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for num in &nums {
            let count = count_map.entry(*num).or_insert(0);
            *count += 1;
        }
        
        // 将哈希表转换为元组的向量，并根据出现次数进行排序

        let mut count_vec: Vec<(i32, i32)> = count_map.into_iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        // 提取前k个元素作为结果

        let result: Vec<i32> = count_vec.into_iter().take(k as usize).map(|(num, _)| num).collect();
        
        result

    }
}
