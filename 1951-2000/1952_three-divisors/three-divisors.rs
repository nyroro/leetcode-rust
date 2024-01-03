
impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut factors = Vec::new();
        
        for i in 1..=n {
            if n % i == 0 {
                factors.push(i);
            }
        }
        
        factors.len() == 3

    }
}
