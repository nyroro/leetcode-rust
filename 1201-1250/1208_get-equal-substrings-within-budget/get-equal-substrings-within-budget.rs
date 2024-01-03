
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let mut start = 0;
        let mut end = 0;
        let mut cost = 0;
        let mut max_length = 0;

        while end < s_chars.len() {
            cost += (s_chars[end] as i32 - t_chars[end] as i32).abs();

            while cost > max_cost {
                cost -= (s_chars[start] as i32 - t_chars[start] as i32).abs();
                start += 1;
            }

            max_length = max_length.max(end - start + 1);
            end += 1;
        }

        max_length as i32

    }
}
