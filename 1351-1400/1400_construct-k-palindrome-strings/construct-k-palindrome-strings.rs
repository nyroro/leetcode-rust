
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut char_count = std::collections::HashMap::new();
        for ch in s.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }
        
        let mut odd_count = 0;
        for count in char_count.values() {
            if count % 2 == 1 {
                odd_count += 1;
            }
        }
        
        odd_count <= k && k <= s.len() as i32

    }
}
