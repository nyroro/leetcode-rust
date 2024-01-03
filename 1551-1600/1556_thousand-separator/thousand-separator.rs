
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut result = String::new();
        let mut count = 0;
        let mut num = n;

        if num == 0 {
            return "0".to_string();
        }

        while num > 0 {
            if count != 0 && count % 3 == 0 {
                result.insert(0, '.');
            }
            result.insert(0, (num % 10 + '0' as i32) as u8 as char);
            num /= 10;
            count += 1;
        }

        result

    }
}
