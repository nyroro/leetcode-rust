
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect(); // 将字符串转换为字符数组

        let mut groups: Vec<String> = chars

            .chunks(k as usize) // 按照指定大小k进行分组

            .map(|chunk| chunk.iter().collect::<String>()) // 将每个分组转换为字符串

            .collect();

        // 处理最后一个分组，如果长度不足k，则添加fill字符

        let last_group_len = groups.last().unwrap().len();
        if last_group_len < k as usize {
            let fill_chars = (k as usize - last_group_len).min(k as usize);
            groups.last_mut().unwrap().push_str(&fill.to_string().repeat(fill_chars));
        }

        groups

    }
}
