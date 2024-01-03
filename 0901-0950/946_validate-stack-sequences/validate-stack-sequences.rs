
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut pop_index = 0;

        for num in pushed {
            stack.push(num);

            while let Some(&top) = stack.last() {
                if top == popped[pop_index] {
                    stack.pop();
                    pop_index += 1;
                } else {
                    break;
                }
            }
        }

        stack.is_empty()
    }
}
