
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut start = 0;
        let mut end = 0;
        
        while end < n {
            if chars[start] != chars[end] {
                if end - start >= 3 {
                    result.push(vec![start as i32, (end - 1) as i32]);
                }
                start = end;
            }
            end += 1;
        }
        
        if end - start >= 3 {
            result.push(vec![start as i32, (end - 1) as i32]);
        }
        
        result

    }
}
