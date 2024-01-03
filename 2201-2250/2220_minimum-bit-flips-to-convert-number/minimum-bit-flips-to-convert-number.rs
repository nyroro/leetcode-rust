
impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut flips = 0;
        let mut start = start;
        let mut goal = goal;
        
        while start != goal {
            if start & 1 != goal & 1 {
                flips += 1;
                start ^= 1;
            }
            start >>= 1;
            goal >>= 1;
        }
        
        flips

    }
}
