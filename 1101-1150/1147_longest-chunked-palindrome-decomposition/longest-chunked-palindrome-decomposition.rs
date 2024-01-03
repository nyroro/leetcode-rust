
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        // 递归终止条件

        if text.is_empty() {
            return 0;
        }
        
        let n = text.len();
        let mut result = 1;
        
        // 遍历字符串的前半部分

        for i in 1..=n/2 {
            let prefix = &text[..i];
            let suffix = &text[n-i..];
            
            // 如果找到相等的子字符串

            if prefix == suffix {
                // 递归调用函数计算剩余部分的最大子字符串数量

                result = result.max(2 + Solution::longest_decomposition(text[i..n-i].to_string()));
                break;
            }
        }
        
        result

    }
}
