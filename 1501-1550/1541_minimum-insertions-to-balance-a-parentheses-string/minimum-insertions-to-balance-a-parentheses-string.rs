


impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut insertions = 0;
        let mut i = 0;
        let chars: Vec<char> = s.chars().collect();
        
        while i < chars.len() {
            if chars[i] == '(' {
                stack.push(i);
                i += 1;
            } else {
                if i + 1 < chars.len() && chars[i + 1] == ')' {
                    i += 2;
                } else {
                    insertions += 1;
                    i += 1;
                }
                if stack.len() > 0 {
                    stack.pop();
                } else {
                    insertions += 1;
                }
            }
        }
        
        insertions + (stack.len() as i32 * 2)
    }
}

fn main() {
    let s1 = "(()))".to_string();
    let s2 = "())".to_string();
    let s3 = "))())(".to_string();
    println!("{}", Solution::min_insertions(s1)); // Output: 1

    println!("{}", Solution::min_insertions(s2)); // Output: 0

    println!("{}", Solution::min_insertions(s3)); // Output: 3

}
