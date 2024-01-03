
impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        // Convert the integer to a string

        let n_str = n.to_string();
        
        // Concatenate n, 2*n, and 3*n

        let concatenated_str = format!("{}{}{}", n_str, (2 * n), (3 * n));
        
        // Check if the length of the concatenated string is 9

        if concatenated_str.len() != 9 {
            return false;
        }
        
        // Check if the concatenated string contains all digits from 1 to 9

        let mut digits = [false; 9];
        for c in concatenated_str.chars() {
            if c == '0' || c > '9' {
                return false;
            }
            let index = (c as u8 - b'1') as usize;
            digits[index] = true;
        }
        
        digits.iter().all(|&x| x)
    }
}
