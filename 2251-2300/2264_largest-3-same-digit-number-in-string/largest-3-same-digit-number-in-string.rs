
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut result = String::new();
        let num_chars: Vec<char> = num.chars().collect();
        
        for i in 0..num_chars.len() - 2 {
            if num_chars[i] == num_chars[i + 1] && num_chars[i] == num_chars[i + 2] {
                let good_integer = num_chars[i].to_string().repeat(3);
                if good_integer > result {
                    result = good_integer;
                }
            }
        }
        
        result

    }
}
