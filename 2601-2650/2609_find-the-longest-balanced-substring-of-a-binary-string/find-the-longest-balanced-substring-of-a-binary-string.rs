
impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut temp = "01".to_string();
        while s.contains(&temp) {
            temp = format!("0{}1", temp);
        }
        (temp.len() - 2) as i32

    }
}
