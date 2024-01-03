
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut max_vowels = 0;
        let mut vowels_count = 0;
        
        // 统计初始窗口内的元音字母数量

        for i in 0..k as usize {
            if Self::is_vowel(s[i] as char) {
                vowels_count += 1;
            }
        }
        max_vowels = vowels_count;

        // 滑动窗口统计最大元音字母数量

        for i in k as usize..s.len() {
            if Self::is_vowel(s[i] as char) {
                vowels_count += 1;
            }
            if Self::is_vowel(s[i - k as usize] as char) {
                vowels_count -= 1;
            }
            max_vowels = max_vowels.max(vowels_count);
        }
        
        max_vowels

    }
    
    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }
}
