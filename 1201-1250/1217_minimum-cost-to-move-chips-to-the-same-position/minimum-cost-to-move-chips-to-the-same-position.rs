
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd_count = 0;
        let mut even_count = 0;
        
        for chip in position {
            if chip % 2 == 0 {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }
        
        if odd_count < even_count {
            return odd_count;
        } else {
            return even_count;
        }
    }
}
