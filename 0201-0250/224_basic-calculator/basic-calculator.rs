
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut num = 0;
        let mut sign = 1;
        let mut result = 0;

        for c in s.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else if c == '+' || c == '-' {
                result += sign * num;
                num = 0;
                sign = if c == '+' { 1 } else { -1 };
            } else if c == '(' {
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            } else if c == ')' {
                result += sign * num;
                num = 0;
                result *= stack.pop().unwrap();
                result += stack.pop().unwrap();
            }
        }

        result += sign * num;

        result

    }
}
