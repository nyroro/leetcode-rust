
impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 创建辅助函数，检查字符串是否以元音字母开头和结尾

        fn is_vowel_string(s: &str) -> bool {
            let first_char = s.chars().next().unwrap();
            let last_char = s.chars().last().unwrap();
            let vowels = ['a', 'e', 'i', 'o', 'u'];
            vowels.contains(&first_char) && vowels.contains(&last_char)
        }
        
        // 创建前缀和数组

        let mut prefix_sum = vec![0; words.len() + 1];
        for i in 1..=words.len() {
            prefix_sum[i] = prefix_sum[i - 1] + if is_vowel_string(&words[i - 1]) { 1 } else { 0 };
        }
        
        // 遍历查询数组，计算每个查询的结果

        let mut result = Vec::new();
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            result.push(prefix_sum[right + 1] - prefix_sum[left]);
        }
        
        result

    }
}
