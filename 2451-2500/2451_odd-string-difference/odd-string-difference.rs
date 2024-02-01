
impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        // 创建一个变量用于存储结果

        let mut result = String::new();
        
        // 遍历字符串数组

        for i in 0..words.len() {
            // 计算差值数组

            let mut difference = vec![];
            let chars: Vec<char> = words[i].chars().collect();
            for j in 0..chars.len() - 1 {
                let diff = chars[j + 1] as i32 - chars[j] as i32;
                difference.push(diff);
            }
            
            // 检查差值数组是否与其他字符串相同

            let mut is_odd = true;
            for k in 0..words.len() {
                if k != i {
                    let mut other_difference = vec![];
                    let other_chars: Vec<char> = words[k].chars().collect();
                    for l in 0..other_chars.len() - 1 {
                        let diff = other_chars[l + 1] as i32 - other_chars[l] as i32;
                        other_difference.push(diff);
                    }
                    if difference == other_difference {
                        is_odd = false;
                        break;
                    }
                }
            }
            
            // 如果差值数组不同，则将当前字符串赋给结果并跳出循环

            if is_odd {
                result = words[i].to_string();
                break;
            }
        }
        
        // 返回结果

        result

    }
}
