
impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        // 初始化最大值为0

        let mut max_value = 0;
        
        // 遍历字符串数组

        for s in strs {
            // 初始化当前字符串的值为0

            let mut current_value = 0;
            
            // 判断字符串是否只包含数字

            if s.chars().all(char::is_numeric) {
                // 如果是，将字符串转换为整数

                current_value = s.parse::<i32>().unwrap();
            } else {
                // 如果不是，字符串的值为其长度

                current_value = s.len() as i32;
            }
            
            // 更新最大值

            if current_value > max_value {
                max_value = current_value;
            }
        }
        
        // 返回最大值

        max_value

    }
}
