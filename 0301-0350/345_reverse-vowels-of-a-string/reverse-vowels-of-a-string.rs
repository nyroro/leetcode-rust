
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut chars: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0, chars.len() - 1);
        
        while left < right {
            if vowels.contains(&chars[left]) && vowels.contains(&chars[right]) {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            } else if vowels.contains(&chars[left]) {
                right -= 1;
            } else {
                left += 1;
            }
        }
        
        chars.into_iter().collect()
    }
}
