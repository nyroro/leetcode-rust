
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut freq = vec![0; 26];
        let mut max_count = 0;
        let mut left = 0;
        let mut result = 0;

        for right in 0..s.len() {
            freq[(s[right] as u8 - b'A') as usize] += 1;
            max_count = max_count.max(freq[(s[right] as u8 - b'A') as usize]);

            if (right - left + 1) as i32 - max_count > k {
                freq[(s[left] as u8 - b'A') as usize] -= 1;
                left += 1;
            }

            result = result.max(right - left + 1);
        }

        result as i32

    }
}
