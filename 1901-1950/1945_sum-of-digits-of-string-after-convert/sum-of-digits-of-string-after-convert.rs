
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        // Convert the string to a sequence of numbers

        let mut num_str = String::new();
        for ch in s.chars() {
            let num = (ch as u8 - b'a' + 1).to_string();
            num_str.push_str(&num);
        }
        
        // Perform the transformation k times

        let mut result = 0;
        for _ in 0..k {
            result = 0;
            for ch in num_str.chars() {
                result += ch.to_digit(10).unwrap() as i32;
            }
            num_str = result.to_string();
        }
        
        result

    }
}
