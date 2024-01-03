
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        
        for operation in operations {
            if operation == "++X" || operation == "X++" {
                x += 1;
            } else if operation == "--X" || operation == "X--" {
                x -= 1;
            }
        }
        
        x

    }
}
