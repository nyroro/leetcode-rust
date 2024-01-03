
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut i = num1.len() as i32 - 1;
        let mut j = num2.len() as i32 - 1;
        let mut carry = 0;
        let mut result = String::new();

        while i >= 0 || j >= 0 {
            let digit1 = if i >= 0 {
                num1.chars().nth(i as usize).unwrap().to_digit(10).unwrap() as i32

            } else {
                0

            };
            let digit2 = if j >= 0 {
                num2.chars().nth(j as usize).unwrap().to_digit(10).unwrap() as i32

            } else {
                0

            };

            let sum = digit1 + digit2 + carry;
            let digit = sum % 10;
            carry = sum / 10;

            result.insert(0, std::char::from_digit(digit as u32, 10).unwrap());

            i -= 1;
            j -= 1;
        }

        if carry != 0 {
            result.insert(0, std::char::from_digit(carry as u32, 10).unwrap());
        }

        result

    }
}
