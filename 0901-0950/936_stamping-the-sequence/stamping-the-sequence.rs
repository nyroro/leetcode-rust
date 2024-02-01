
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp: Vec<char> = stamp.chars().collect();
        let target: Vec<char> = target.chars().collect();
        let mut result: Vec<i32> = Vec::new();
        let mut visited: Vec<bool> = vec![false; target.len()];
        let mut stars = 0;
        
        while stars < target.len() {
            let mut done_replacement = false;
            
            for i in 0..=target.len()-stamp.len() {
                if !visited[i] && Self::can_replace(&target[i..], &stamp) {
                    stars += Self::replace(&mut target[i..], &stamp);
                    visited[i] = true;
                    result.push(i as i32);
                    done_replacement = true;
                    
                    if stars == target.len() {
                        break;
                    }
                }
            }
            
            if !done_replacement {
                return Vec::new();
            }
        }
        
        result.reverse();
        result

    }
    
    fn can_replace(target: &[char], stamp: &[char]) -> bool {
        for i in 0..stamp.len() {
            if target[i] != '?' && target[i] != stamp[i] {
                return false;
            }
        }
        true

    }
    
    fn replace(target: &mut [char], stamp: &[char]) -> usize {
        let mut stars = 0;
        for i in 0..stamp.len() {
            if target[i] != '?' {
                target[i] = '?';
                stars += 1;
            }
        }
        stars

    }
}
