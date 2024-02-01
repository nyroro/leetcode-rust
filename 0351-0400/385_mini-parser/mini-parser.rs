
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if s.chars().all(|c| c.is_digit(10) || c == '-') {
            return NestedInteger::Int(s.parse().unwrap());
        }

        let mut stack: Vec<NestedInteger> = Vec::new();
        let mut num = 0;
        let mut is_negative = false;
        let mut is_num = false;

        for c in s.chars() {
            match c {
                '[' => {
                    let nested = NestedInteger::List(Vec::new());
                    stack.push(nested);
                }
                '0'..='9' => {
                    num = num * 10 + c.to_digit(10).unwrap() as i32;
                    is_num = true;
                }
                '-' => {
                    is_negative = true;
                }
                ',' => {
                    if is_num {
                        let last = stack.last_mut().unwrap();
                        match last {
                            NestedInteger::List(list) => {
                                list.push(NestedInteger::Int(if is_negative { -num } else { num }));
                            }
                            _ => {}
                        }
                        num = 0;
                        is_negative = false;
                        is_num = false;
                    }
                }
                ']' => {
                    if is_num {
                        let last = stack.last_mut().unwrap();
                        match last {
                            NestedInteger::List(list) => {
                                list.push(NestedInteger::Int(if is_negative { -num } else { num }));
                            }
                            _ => {}
                        }
                        num = 0;
                        is_negative = false;
                        is_num = false;
                    }
                    let nested = stack.pop().unwrap();
                    if let Some(last) = stack.last_mut() {
                        match last {
                            NestedInteger::List(list) => {
                                list.push(nested);
                            }
                            _ => {}
                        }
                    } else {
                        return nested;
                    }
                }
                _ => {}
            }
        }

        // 返回栈中唯一的元素

        stack.pop().unwrap()
    }
}
