


impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let mut max_sum: i64 = 0;
        
        for i in 0..max_heights.len() {
            let mut curr: i64 = max_heights[i] as i64;
            let mut v: i64 = max_heights[i] as i64;
            
            for j in (i + 1)..max_heights.len() {
                v = v.min(max_heights[j] as i64);
                curr += v;
            }
            
            v = max_heights[i] as i64;
            
            for j in (0..i).rev() {
                v = v.min(max_heights[j] as i64);
                curr += v;
            }
            
            max_sum = max_sum.max(curr);
        }
        
        max_sum

    }
}
