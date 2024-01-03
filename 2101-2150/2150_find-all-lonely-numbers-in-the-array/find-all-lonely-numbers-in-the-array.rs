
impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        // 创建一个哈希表来存储数字及其出现次数

        let mut count_map = std::collections::HashMap::new();
        let mut result = Vec::new();

        // 遍历数组，将数字及其出现次数存储到哈希表中

        for &num in nums.iter() {
            *count_map.entry(num).or_insert(0) += 1;
        }

        // 再次遍历数组，检查每个数字是否为“孤立数字”
        for &num in nums.iter() {
            if count_map[&num] == 1 && !count_map.contains_key(&(num - 1)) && !count_map.contains_key(&(num + 1)) {
                result.push(num);
            }
        }

        result

    }
}
