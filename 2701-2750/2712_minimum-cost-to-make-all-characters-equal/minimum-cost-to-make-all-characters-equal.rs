
impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        let s = s.chars().collect::<Vec<char>>();
        let mut cost = 0;
        
        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                cost += std::cmp::min(i, s.len() - i) as i64;
            }
        }
        
        cost

    }
}
