
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let len1 = s1_chars.len();
        let len2 = s2_chars.len();
        let mut count1 = 0;
        let mut count2 = 0;
        let mut i = 0;
        let mut j = 0;
        
        while count1 < n1 {
            if s1_chars[i] == s2_chars[j] {
                j += 1;
                if j == len2 {
                    j = 0;
                    count2 += 1;
                }
            }
            i += 1;
            if i == len1 {
                i = 0;
                count1 += 1;
            }
        }
        
        count2 / n2

    }
}
