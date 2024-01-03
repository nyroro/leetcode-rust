
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        // 计算每个数字的频率

        let mut freq_map: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        
        // 根据频率和值进行排序

        let mut sorted_nums: Vec<(i32, usize)> = freq_map.into_iter().collect();
        sorted_nums.sort_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        
        // 转换为只包含数字的数组

        let result: Vec<i32> = sorted_nums.into_iter().flat_map(|(num, freq)| vec![num; freq]).collect();
        
        result

    }
}
