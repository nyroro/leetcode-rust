
impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut max_num = 0;
        let mut result = String::new();
        
        for (i, c) in number.chars().enumerate() {
            if c == digit {
                let new_num: String = number.chars().take(i).chain(number.chars().skip(i + 1)).collect();
                let new_num_int: i32 = new_num.parse().unwrap_or(0);
                if new_num_int > max_num {
                    max_num = new_num_int;
                    result = new_num;
                }
            }
        }
        
        result

    }
}
