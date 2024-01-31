
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut ones: Vec<char> = Vec::new();
        
        for (i, &c) in chars.iter().enumerate() {
            if c == '1' {
                ones.push('1');
                chars.remove(i);
                break;
            }
        }
        
        chars.sort_by(|a, b| b.cmp(a));
        chars.extend(ones);
        
        chars.into_iter().collect()
    }
}
