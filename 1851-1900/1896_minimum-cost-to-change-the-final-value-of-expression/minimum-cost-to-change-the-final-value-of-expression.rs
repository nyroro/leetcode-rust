


impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let mut num_stack: Vec<i32> = Vec::new();
        let mut op_stack: Vec<i32> = Vec::new();
        let mut sign_stack: Vec<char> = Vec::new();
        let chars: Vec<char> = expression.chars().collect();
        let length = chars.len();

        for i in 0..length {
            let c = chars[i];
            if c.is_digit(10) {
                num_stack.push((c.to_digit(10).unwrap() as i32));
                op_stack.push(1);
            } else if c == ')' {
                sign_stack.pop();
            } else {
                sign_stack.push(c);
                continue;
            }

            if num_stack.len() > 1 && sign_stack.last() != Some(&'(') {
                let num2 = num_stack.pop().unwrap();
                let num1 = num_stack.pop().unwrap();
                let op2 = op_stack.pop().unwrap();
                let op1 = op_stack.pop().unwrap();
                let sign = sign_stack.pop().unwrap();
                let ops = Solution::min_op(num1, num2, op1, op2, sign);
                num_stack.push(ops[0]);
                op_stack.push(ops[1]);
            }
        }
        op_stack.pop().unwrap()
    }

    fn min_op(num1: i32, num2: i32, op1: i32, op2: i32, sign: char) -> [i32; 2] {
        if sign == '&' {
            if num1 == 1 && num2 == 1 {
                return [1, op1.min(op2)];
            } else if num1 == 0 && num2 == 0 {
                return [0, op1.min(op2) + 1];
            } else {
                return [0, 1];
            }
        } else {
            if num1 == 0 && num2 == 0 {
                return [0, op1.min(op2)];
            } else if num1 == 1 && num2 == 1 {
                return [1, op1.min(op2) + 1];
            } else {
                return [1, 1];
            }
        }
    }
}
