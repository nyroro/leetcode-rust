
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut length = (area as f64).sqrt() as i32;
        
        while area % length != 0 {
            length -= 1;
        }
        
        let width = area / length;
        
        if length < width {
            vec![width, length]
        } else {
            vec![length, width]
        }
    }
}
