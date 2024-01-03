
impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let trimmed_num = num.trim_end_matches('0').to_string();
        trimmed_num

    }
}
