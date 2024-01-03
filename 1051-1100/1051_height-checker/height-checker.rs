
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut expected = heights.clone();
        expected.sort();
        
        for i in 0..heights.len() {
            if heights[i] != expected[i] {
                count += 1;
            }
        }
        
        count

    }
}
