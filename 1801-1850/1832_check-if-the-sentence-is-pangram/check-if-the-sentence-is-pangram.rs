
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        // 创建一个长度为26的布尔数组，用于记录每个字母是否出现过

        let mut alphabet = [false; 26];

        // 遍历给定的字符串

        for ch in sentence.chars() {
            // 将字符转换为索引，范围为0-25

            let index = (ch as u8 - b'a') as usize;
            // 将对应的布尔数组位置设置为true

            alphabet[index] = true;
        }

        // 检查布尔数组中是否还有false值

        alphabet.iter().all(|&x| x)
    }
}
