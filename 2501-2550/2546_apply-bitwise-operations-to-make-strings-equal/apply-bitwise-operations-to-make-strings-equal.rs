
impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        if s.contains('1') && !target.contains('1') || target.contains('1') && !s.contains('1') {
            return false;
        }
        true

    }
}
