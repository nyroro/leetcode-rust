
impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut result = String::new();
        let mut num = n;

        while num != 0 {
            let bit = num & 1;
            result.insert(0, std::char::from_digit(bit as u32, 10).unwrap());
            if bit == 1 && num != 1 {
                num -= 1;
            }
            num /= -2;
        }

        if result.is_empty() {
            result.push('0');
        }

        result

    }
}
