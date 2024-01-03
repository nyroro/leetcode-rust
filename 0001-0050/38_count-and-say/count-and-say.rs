
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n <= 0 {
            return String::new();
        }
        if n == 1 {
            return String::from("1");
        }
        let prev = Solution::count_and_say(n - 1);
        let mut result = String::new();
        let mut count = 1;
        let mut prev_char = prev.chars().next().unwrap();
        for ch in prev.chars().skip(1) {
            if ch == prev_char {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(prev_char);
                count = 1;
                prev_char = ch;
            }
        }
        result.push_str(&count.to_string());
        result.push(prev_char);
        result

    }
}
