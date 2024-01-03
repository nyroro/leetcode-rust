
impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut answer = vec![];
        let mut depth = 0;
        
        for c in seq.chars() {
            if c == '(' {
                depth += 1;
                answer.push(depth % 2);
            } else {
                answer.push(depth % 2);
                depth -= 1;
            }
        }
        
        answer

    }
}
