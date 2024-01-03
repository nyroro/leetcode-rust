
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_area = 0;
        
        while l < r {
            let width = (r - l) as i32;
            let h = height[l].min(height[r]);
            let area = width * h;
            max_area = max_area.max(area);
            
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        
        max_area

    }
}
