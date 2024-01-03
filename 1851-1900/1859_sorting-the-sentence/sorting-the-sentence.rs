
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        // 步骤 1：创建一个空的字符串数组 words 用于存储分割后的单词

        let mut words: Vec<&str> = Vec::new();
        
        // 步骤 2：将输入的字符串 s 按空格分割成单词，并存储在 words 数组中

        for word in s.split_whitespace() {
            words.push(word);
        }
        
        // 步骤 3：对 words 数组中的单词进行排序，排序的依据是单词中的数字部分

        words.sort_by_key(|word| word.chars().last().unwrap());
        
        // 步骤 4：遍历排序后的 words 数组，去除每个单词中的数字部分

        let original_words: Vec<String> = words.iter().map(|word| word.chars().take(word.len() - 1).collect()).collect();
        
        // 步骤 5：将处理后的单词列表重新连接成原始的句子

        original_words.join(" ")
    }
}
