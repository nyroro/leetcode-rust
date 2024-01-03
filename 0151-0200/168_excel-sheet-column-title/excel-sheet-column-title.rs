
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = String::new();
        let mut n = column_number;
        
        while n > 0 {
            let remainder = (n - 1) % 26;
            let letter = (remainder as u8 + b'A') as char;
            result.insert(0, letter);
            n = (n - 1) / 26;
        }
        
        result

    }
}
