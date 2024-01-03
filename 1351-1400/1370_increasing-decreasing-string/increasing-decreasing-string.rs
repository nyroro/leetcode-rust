
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut count = [0; 26]; // 创建一个长度为 26 的数组，用于记录每个字符的出现次数

        let mut result = String::new(); // 创建一个空字符串，用于存储排序后的结果


        // 统计每个字符的出现次数

        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        while result.len() < s.len() {
            // 从小到大遍历字符

            for i in 0..26 {
                if count[i] > 0 {
                    result.push((i as u8 + b'a') as char);
                    count[i] -= 1;
                }
            }

            // 从大到小遍历字符

            for i in (0..26).rev() {
                if count[i] > 0 {
                    result.push((i as u8 + b'a') as char);
                    count[i] -= 1;
                }
            }
        }

        result

    }
}
