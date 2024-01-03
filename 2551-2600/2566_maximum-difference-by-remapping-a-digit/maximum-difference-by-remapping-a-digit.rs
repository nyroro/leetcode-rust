
impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let num_str = num.to_string(); // 将数字转换为字符串

        let digits: Vec<char> = num_str.chars().collect(); // 将字符串转换为字符数组
        
        let mut min_diff = i32::MAX; // 初始化最小差异为最大值

        let mut max_diff = 0; // 初始化最大差异为0
        
        // 遍历每一位数字

        for i in 0..digits.len() {
            let digit = digits[i].to_digit(10).unwrap() as i32; // 将字符转换为数字
            
            // 尝试将当前数字替换为0到9之间的每个数字

            for j in 0..=9 {
                let replaced_num = num_str.replace(digit.to_string().as_str(), j.to_string().as_str()); // 替换数字
                
                let replaced_val = replaced_num.parse::<i32>().unwrap(); // 将替换后的字符串转换为数字
                
                // 更新最小差异和最大差异

                min_diff = min_diff.min(replaced_val);
                max_diff = max_diff.max(replaced_val);
            }
        }
        
        max_diff - min_diff // 计算最大差异和最小差异之间的差异

    }
}
