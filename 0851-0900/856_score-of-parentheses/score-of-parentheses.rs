
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut score = 0;

        for c in s.chars() {
            if c == '(' {
                stack.push(score);
                score = 0;
            } else {
                let prev_score = stack.pop().unwrap();
                score = prev_score + std::cmp::max(score * 2, 1);
            }
        }

        score

    }
}
