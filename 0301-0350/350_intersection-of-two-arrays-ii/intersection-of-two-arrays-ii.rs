
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();
        
        // 遍历第一个数组，记录每个元素的出现次数

        for num in nums1 {
            *count_map.entry(num).or_insert(0) += 1;
        }
        
        // 遍历第二个数组，查找并添加相同元素到结果数组中

        for num in nums2 {
            if let Some(count) = count_map.get_mut(&num) {
                if *count > 0 {
                    result.push(num);
                    *count -= 1;
                }
            }
        }
        
        result

    }
}
