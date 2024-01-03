
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // 定义数字到字母的映射关系

        let mapping: Vec<&str> = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"
        ];
        
        // 存储结果的向量

        let mut result: Vec<String> = Vec::new();
        
        // 辅助函数，用于生成所有可能的字母组合

        fn backtrack(
            digits: &str,
            mapping: &Vec<&str>,
            combination: &mut String,
            index: usize,
            result: &mut Vec<String>
        ) {
            // 如果已经遍历完所有数字，将当前组合加入结果向量

            if index == digits.len() {
                result.push(combination.clone());
                return;
            }
            
            // 获取当前数字对应的字母集合

            let letters = mapping[digits.chars().nth(index).unwrap() as usize - '0' as usize];
            
            // 遍历当前数字对应的字母集合

            for letter in letters.chars() {
                // 将当前字母加入组合

                combination.push(letter);
                // 递归生成下一个数字的字母组合

                backtrack(digits, mapping, combination, index + 1, result);
                // 回溯，将当前字母从组合中移除

                combination.pop();
            }
        }
        
        // 调用辅助函数生成所有可能的字母组合

        if !digits.is_empty() {
            let mut combination = String::new();
            backtrack(&digits, &mapping, &mut combination, 0, &mut result);
        }
        
        result

    }
}
