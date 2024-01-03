


impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = expression.chars().collect();
        while i < chars.len() {
            let c = chars[i];
            if c == ')' {
                let mut seen_false = false;
                let mut seen_true = false;
                while let Some(top) = stack.pop() {
                    if top == 't' {
                        seen_true = true;
                    } else if top == 'f' {
                        seen_false = true;
                    } else if top == '&' || top == '|' {
                        if (top == '&' && !seen_false) || (top == '|' && seen_true) {
                            stack.push('t');
                        } else {
                            stack.push('f');
                        }
                        break;
                    } else if top == '!' {
                        stack.push(if seen_true { 'f' } else { 't' });
                        break;
                    }
                }
            } else if c != ',' && c != '(' {
                stack.push(c);
            }
            i += 1;
        }
        stack[0] == 't'
    }
}
