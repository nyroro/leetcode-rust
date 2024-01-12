
impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut max_num = String::new();
        
        for (i, c) in number.chars().enumerate() {
            if c == digit {
                let new_num: String = number.chars().take(i).chain(number.chars().skip(i + 1)).collect();
                if new_num > max_num {
                    max_num = new_num;
                }
            }
        }
        
        max_num

    }
}
