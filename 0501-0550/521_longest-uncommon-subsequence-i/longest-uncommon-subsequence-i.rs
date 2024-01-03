
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            return -1;
        } else {
            return std::cmp::max(a.len(), b.len()) as i32;
        }
    }
}
