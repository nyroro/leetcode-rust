
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewel_set: std::collections::HashSet<char> = jewels.chars().collect(); // 将宝石字符串转换为字符集合

        let count = stones.chars().filter(|c| jewel_set.contains(c)).count(); // 过滤出是宝石的字符并计数

        count as i32 // 将计数转换为 i32 类型并返回

    }
}
