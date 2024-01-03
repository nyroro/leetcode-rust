
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;
        let mut power = 0;
        
        for c in column_title.chars().rev() {
            let digit = (c as u8 - b'A' + 1) as i32;
            result += digit * i32::pow(26, power);
            power += 1;
        }
        
        result

    }
}
