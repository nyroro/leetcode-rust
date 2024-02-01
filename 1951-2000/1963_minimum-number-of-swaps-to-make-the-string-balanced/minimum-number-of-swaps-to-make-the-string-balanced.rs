
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut swaps = 0;

        for c in s.chars() {
            if c == '[' {
                stack.push(c);
            } else {
                if let Some(top) = stack.last() {
                    if *top == '[' {
                        stack.pop();
                    } else {
                        stack.push(c);
                        swaps += 1;
                    }
                } else {
                    stack.push(c);
                    swaps += 1;
                }
            }
        }

        swaps / 2

    }
}
