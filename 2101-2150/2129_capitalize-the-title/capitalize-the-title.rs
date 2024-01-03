
impl Solution {
    pub fn capitalize_title(title: String) -> String {
        // 创建一个空字符串变量result，用于存储处理后的结果

        let mut result = String::new();
        
        // 将输入的字符串title按空格分割成单词，并存储在一个数组中

        let words: Vec<&str> = title.split(' ').collect();
        
        // 遍历每个单词，根据长度进行大写化处理，然后将处理后的单词添加到result中

        for (index, word) in words.iter().enumerate() {
            if index > 0 {
                result.push(' '); // 添加空格

            }
            if word.len() <= 2 {
                result.push_str(&word.to_lowercase()); // 将长度为1或2的单词全部转换为小写

            } else {
                let capitalized_word = word[..1].to_uppercase() + &word[1..].to_lowercase(); // 将长度大于2的单词首字母大写，其余字母小写

                result.push_str(&capitalized_word);
            }
        }
        
        // 返回处理后的result字符串

        result

    }
}
