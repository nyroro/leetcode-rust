
impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut s = s;
        let mut seconds = 0;
        
        while s.contains("01") {
            s = s.replace("01", "10");
            seconds += 1;
        }
        
        seconds

    }
}
