
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>(); // 将字符串转换为字符数组

        let mut count = 0;
        let mut prev_count = 0;
        let mut curr_count = 1;
        
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                curr_count += 1;
            } else {
                count += std::cmp::min(prev_count, curr_count);
                prev_count = curr_count;
                curr_count = 1;
            }
        }
        
        count += std::cmp::min(prev_count, curr_count);
        
        count

    }
}
