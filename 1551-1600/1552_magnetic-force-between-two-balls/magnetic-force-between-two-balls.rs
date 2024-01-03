
impl Solution {
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut pos = position.clone();
        pos.sort();
        
        let mut left = 1;
        let mut right = pos[pos.len() - 1] - pos[0];
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if Self::check(mid, &pos, m) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        right

    }
    
    fn check(min_force: i32, pos: &Vec<i32>, m: i32) -> bool {
        let mut count = 1;
        let mut prev = pos[0];
        
        for i in 1..pos.len() {
            if pos[i] - prev >= min_force {
                count += 1;
                prev = pos[i];
            }
        }
        
        count >= m

    }
}
