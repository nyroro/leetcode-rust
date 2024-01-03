
impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        // 分割字符串得到单词列表

        let words: Vec<&str> = s.split_whitespace().collect();
        
        // 找到最长单词的长度

        let max_len = words.iter().map(|word| word.len()).max().unwrap();
        
        // 创建二维字符数组

        let mut columns: Vec<Vec<char>> = vec![vec![' '; words.len()]; max_len];
        
        // 将单词的字符放入对应的列中

        for (i, word) in words.iter().enumerate() {
            let chars: Vec<char> = word.chars().collect();
            for (j, &c) in chars.iter().enumerate() {
                columns[j][i] = c;
            }
        }
        
        // 将每一列的字符数组转换为字符串

        let mut result: Vec<String> = Vec::new();
        for column in columns {
            let s: String = column.into_iter().collect();
            result.push(s.trim_end().to_string());
        }
        
        result

    }
}
