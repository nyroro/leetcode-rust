
impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        // 辅助函数，计算字符串中字典序最小字符的频率

        fn f(s: &str) -> i32 {
            let mut freq = [0; 26];
            let mut min_char = 'z';
            for ch in s.chars() {
                freq[ch as usize - 'a' as usize] += 1;
                if ch < min_char {
                    min_char = ch;
                }
            }
            freq[min_char as usize - 'a' as usize]
        }
        
        let mut answer = Vec::new();
        for query in queries {
            let query_freq = f(&query);
            let mut count = 0;
            for word in &words {
                let word_freq = f(word);
                if query_freq < word_freq {
                    count += 1;
                }
            }
            answer.push(count);
        }
        answer

    }
}
