
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        
        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push(num);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                
                match token.as_str() {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "*" => stack.push(a * b),
                    "/" => stack.push(a / b),
                    _ => panic!("Invalid operator"),
                }
            }
        }
        
        stack.pop().unwrap()
    }
}
