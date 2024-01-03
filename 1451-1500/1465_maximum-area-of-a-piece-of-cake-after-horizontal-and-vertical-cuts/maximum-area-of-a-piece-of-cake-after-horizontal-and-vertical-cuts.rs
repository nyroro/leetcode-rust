
impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let modulo: i64 = 1000000007;
        
        let mut h_cuts = horizontal_cuts.clone();
        let mut v_cuts = vertical_cuts.clone();
        
        h_cuts.push(0);
        h_cuts.push(h);
        v_cuts.push(0);
        v_cuts.push(w);
        
        h_cuts.sort();
        v_cuts.sort();
        
        let max_h = Self::max_diff(&h_cuts) as i64;
        let max_v = Self::max_diff(&v_cuts) as i64;
        
        ((max_h * max_v) % modulo) as i32

    }
    
    fn max_diff(cuts: &Vec<i32>) -> i32 {
        let mut max_diff = 0;
        for i in 1..cuts.len() {
            max_diff = max_diff.max(cuts[i] - cuts[i - 1]);
        }
        max_diff

    }
}
