
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let distance = target[0].abs() + target[1].abs();
        
        for ghost in ghosts {
            let ghost_distance = (ghost[0] - target[0]).abs() + (ghost[1] - target[1]).abs();
            if ghost_distance <= distance {
                return false;
            }
        }
        
        true

    }
}
