
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut stack: Vec<NestedInteger> = Vec::new();
        let mut num: Option<i32> = None;
        let mut is_negative = false;

        for c in s.chars() {
            match c {
                '[' => {
                    let nested = NestedInteger::List(Vec::new());
                    stack.push(nested);
                }
                ']' => {
                    if let Some(nested) = stack.pop() {
                        if let Some(num) = num {
                            stack.last_mut().unwrap().add(NestedInteger::Int(num));
                            num = None;
                        }
                        stack.last_mut().unwrap().add(nested);
                    }
                }
                ',' => {
                    if let Some(num) = num {
                        stack.last_mut().unwrap().add(NestedInteger::Int(num));
                        num = None;
                    }
                }
                '-' => {
                    is_negative = true;
                }
                '0'..='9' => {
                    let digit = c.to_digit(10).unwrap() as i32;
                    num = Some(num.unwrap_or(0) * 10 + digit);
                }
                _ => {}
            }
        }

        if let Some(num) = num {
            return NestedInteger::Int(if is_negative { -num } else { num });
        }

        stack.pop().unwrap()
    }
}
