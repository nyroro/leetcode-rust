
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        // 创建一个空的向量来存储满足条件的索引

        let mut result = Vec::new();
        
        // 使用enumerate方法来获取数组元素的索引和值

        for (i, &num) in nums.iter().enumerate() {
            // 使用filter方法筛选出满足条件的索引

            if num == key && nums.iter().enumerate().any(|(j, &n)| (i as i32 - j as i32).abs() <= k && n == key) {
                result.push(i as i32);
            }
        }
        
        // 对向量进行排序

        result.sort_unstable();
        
        // 返回结果

        result

    }
}
