
impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let mut total = 0;
        let mut level_count = 0;
        let mut curr_level = 0;
        
        while total < n {
            level_count += 1;
            curr_level += level_count;
            total += curr_level;
        }
        
        if total == n {
            return curr_level;
        }
        
        total -= curr_level;
        curr_level -= level_count;
        let remain = n - total;
        
        let mut min = (remain as f64).sqrt() as i32;
        while min * (min + 1) / 2 < remain {
            min += 1;
        }
        
        curr_level + min

    }
}
