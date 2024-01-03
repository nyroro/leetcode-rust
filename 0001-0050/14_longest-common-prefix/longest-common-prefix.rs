
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        
        let mut longest_prefix = strs[0].clone();
        
        for i in 1..strs.len() {
            let mut j = 0;
            while j < longest_prefix.len() && j < strs[i].len() && longest_prefix.chars().nth(j) == strs[i].chars().nth(j) {
                j += 1;
            }
            
            longest_prefix = longest_prefix[..j].to_string();
            
            if longest_prefix.is_empty() {
                break;
            }
        }
        
        longest_prefix

    }
}
