
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        
        let s_double = s.clone() + &s;
        
        s_double.contains(&goal)
    }
}
