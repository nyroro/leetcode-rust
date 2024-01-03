
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut start = 0;
        let mut end = 0;
        let mut max_length = 0;
        
        let chars: Vec<char> = s.chars().collect();
        
        while end < chars.len() {
            if !set.contains(&chars[end]) {
                set.insert(chars[end]);
                end += 1;
                max_length = std::cmp::max(max_length, end - start);
            } else {
                set.remove(&chars[start]);
                start += 1;
            }
        }
        
        max_length as i32

    }
}
