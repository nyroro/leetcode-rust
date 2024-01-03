
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // 将 nums1 转换为 HashSet

        let set1: HashSet<_> = nums1.into_iter().collect();
        
        // 创建一个空的结果集合

        let mut result = HashSet::new();
        
        // 遍历 nums2，将与 set1 中的元素相同的元素添加到结果集合中

        for num in nums2 {
            if set1.contains(&num) {
                result.insert(num);
            }
        }
        
        // 将结果集合转换为数组并返回

        result.into_iter().collect()
    }
}
