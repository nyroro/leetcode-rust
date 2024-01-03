
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // 检查长度是否相等

        if word1.len() != word2.len() {
            return false;
        }
        
        // 统计字符频次

        let mut count1 = vec![0; 26];
        let mut count2 = vec![0; 26];
        
        for c in word1.chars() {
            count1[(c as u8 - b'a') as usize] += 1;
        }
        
        for c in word2.chars() {
            count2[(c as u8 - b'a') as usize] += 1;
        }
        
        // 检查字符是否相同

        for i in 0..26 {
            if (count1[i] > 0 && count2[i] == 0) || (count1[i] == 0 && count2[i] > 0) {
                return false;
            }
        }
        
        // 检查字符频次分布是否相同

        count1.sort();
        count2.sort();
        
        count1 == count2

    }
}
