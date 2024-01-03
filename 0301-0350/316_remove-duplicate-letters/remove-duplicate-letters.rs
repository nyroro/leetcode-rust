
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut count: Vec<usize> = vec![0; 26];
        let mut visited: Vec<bool> = vec![false; 26];
        
        for ch in s.chars() {
            let idx = (ch as u8 - b'a') as usize;
            count[idx] += 1;
        }
        
        for ch in s.chars() {
            let idx = (ch as u8 - b'a') as usize;
            count[idx] -= 1;
            
            if visited[idx] {
                continue;
            }
            
            while let Some(&last_ch) = stack.last() {
                let last_idx = (last_ch as u8 - b'a') as usize;
                if ch < last_ch && count[last_idx] > 0 {
                    stack.pop();
                    visited[last_idx] = false;
                } else {
                    break;
                }
            }
            
            stack.push(ch);
            visited[idx] = true;
        }
        
        stack.iter().collect()
    }
}
