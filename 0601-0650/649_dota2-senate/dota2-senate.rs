
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant: Vec<usize> = Vec::new();
        let mut dire: Vec<usize> = Vec::new();
        let n = senate.len();
        
        for (i, c) in senate.chars().enumerate() {
            if c == 'R' {
                radiant.push(i);
            } else {
                dire.push(i);
            }
        }
        
        while !radiant.is_empty() && !dire.is_empty() {
            let r = radiant.remove(0);
            let d = dire.remove(0);
            
            if r < d {
                radiant.push(r + n);
            } else {
                dire.push(d + n);
            }
        }
        
        if radiant.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}
