
// 创建一个名为Solution的结构体



// 实现Solution结构体

impl Solution {
    // 定义一个名为restore_string的公共函数，它接受一个String类型的参数s和一个i32类型的数组indices，并返回一个String类型的字符串

    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        // 创建一个空的字符串变量result，用于存储洗牌后的字符串

        let mut result = String::new();
        
        // 创建一个HashMap，用于存储字符的原始位置和它们在洗牌后的位置

        let mut char_map = std::collections::HashMap::new();
        
        // 使用for循环遍历indices数组

        for i in 0..indices.len() {
            // 将s中的字符根据indices数组中的值放入HashMap中

            char_map.insert(indices[i], s.chars().nth(i).unwrap());
        }
        
        // 使用for循环遍历0到indices数组的长度

        for i in 0..indices.len() {
            // 将HashMap中的字符按照顺序放入result字符串中

            result.push(char_map[&(i as i32)]);
        }
        
        // 返回洗牌后的字符串result

        result

    }
}
