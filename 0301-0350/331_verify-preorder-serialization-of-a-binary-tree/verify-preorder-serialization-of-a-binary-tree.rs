
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let nodes: Vec<&str> = preorder.split(',').collect();
        let mut stack: Vec<&str> = Vec::new();

        for node in nodes {
            if node != "#" {
                stack.push(node);
            } else {
                while stack.len() >= 2 && stack[stack.len() - 1] == "#" && stack[stack.len() - 2] == "#" {
                    stack.pop();
                    stack.pop();
                }
                stack.push(node);
            }
        }

        stack.len() == 1 && stack[0] == "#"
    }
}
