
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut result = Vec::new();
        let mut current = String::new();
        Solution::generate_happy_strings(n, ' ', &mut current, &mut result);
        result.sort();
        if k as usize > result.len() {
            return String::new();
        }
        result[k as usize - 1].clone()
    }
    
    fn generate_happy_strings(n: i32, last_char: char, current: &mut String, result: &mut Vec<String>) {
        if current.len() == n as usize {
            result.push(current.clone());
            return;
        }
        for c in ['a', 'b', 'c'].iter() {
            if *c != last_char {
                current.push(*c);
                Solution::generate_happy_strings(n, *c, current, result);
                current.pop();
            }
        }
    }
}
