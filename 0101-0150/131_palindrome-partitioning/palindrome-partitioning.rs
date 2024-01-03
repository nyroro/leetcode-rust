
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut current_partition = Vec::new();
        Self::dfs(s.as_bytes(), 0, &mut current_partition, &mut result);
        result

    }
    
    fn dfs(s: &[u8], start: usize, current_partition: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if start == s.len() {
            result.push(current_partition.clone());
            return;
        }
        
        for i in start..s.len() {
            if Self::is_palindrome(s, start, i) {
                let substring = String::from_utf8_lossy(&s[start..=i]).to_string();
                current_partition.push(substring);
                Self::dfs(s, i + 1, current_partition, result);
                current_partition.pop();
            }
        }
    }
    
    fn is_palindrome(s: &[u8], start: usize, end: usize) -> bool {
        let mut i = start;
        let mut j = end;
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true

    }
}
