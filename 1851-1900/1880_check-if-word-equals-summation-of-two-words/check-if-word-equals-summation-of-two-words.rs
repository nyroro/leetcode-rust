
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        // Step 1: 计算字符串中每个字母的字母值并连接起来

        fn calculate_numerical_value(word: &str) -> i32 {
            let mut result = String::new();
            for c in word.chars() {
                let value = (c as u8 - b'a') as i32;
                result.push_str(&value.to_string());
            }
            result.parse::<i32>().unwrap()
        }

        // Step 2: 比较给定的三个字符串的数值总和是否相等

        let first_value = calculate_numerical_value(&first_word);
        let second_value = calculate_numerical_value(&second_word);
        let target_value = calculate_numerical_value(&target_word);

        first_value + second_value == target_value

    }
}
