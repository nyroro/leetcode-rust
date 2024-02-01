
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let words: Vec<&str> = sentence.split(' ').collect();
        let mut result = String::new();

        for word in words {
            let mut root = String::new();
            for root_word in &dictionary {
                if word.starts_with(root_word) && (root.is_empty() || root_word.len() < root.len()) {
                    root = root_word.clone();
                }
            }
            result.push_str(&root);
            result.push(' ');
        }

        result.pop(); // 移除最后一个空格

        result

    }
}
