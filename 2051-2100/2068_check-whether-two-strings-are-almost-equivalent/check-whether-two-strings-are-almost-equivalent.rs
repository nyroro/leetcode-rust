
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut freq1 = [0; 26];
        let mut freq2 = [0; 26];
        
        // 计算word1中每个字母的频率

        for ch in word1.chars() {
            let index = (ch as u8 - b'a') as usize;
            freq1[index] += 1;
        }
        
        // 计算word2中每个字母的频率

        for ch in word2.chars() {
            let index = (ch as u8 - b'a') as usize;
            freq2[index] += 1;
        }
        
        // 判断每个字母的频率差异是否小于等于3

        for i in 0..26 {
            if (freq1[i] as i32 - freq2[i] as i32).abs() > 3 {
                return false;
            }
        }
        
        true

    }
}
