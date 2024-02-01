
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i64 {
        let mut count = 0;
        for i in 0..s.len() {
            for j in i+1..=s.len() {
                if Solution::is_beautiful(&s[i..j], k) {
                    count += 1;
                }
            }
        }
        count

    }
    
    fn is_beautiful(s: &str, k: i32) -> bool {
        let vowels = s.chars().filter(|&c| "aeiou".contains(c)).count() as i32;
        let consonants = s.len() as i32 - vowels;
        vowels == consonants && vowels * consonants % k == 0

    }
}
