
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        // 创建一个函数来检查两个单词是否满足循环条件

        fn is_circular_word(word1: &str, word2: &str) -> bool {
            word1.chars().last() == word2.chars().next()
        }
        
        // 将输入的句子分割成单词

        let words: Vec<&str> = sentence.split_whitespace().collect();
        
        // 依次检查相邻的单词是否满足循环条件

        for i in 0..words.len() - 1 {
            if !is_circular_word(words[i], words[i + 1]) {
                return false;
            }
        }
        
        // 检查最后一个单词的最后一个字符是否与第一个单词的第一个字符相匹配

        is_circular_word(words.last().unwrap(), words.first().unwrap())
    }
}
