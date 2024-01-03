
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_len = 0;
        let mut count = 0;
        
        for rectangle in rectangles {
            let min_side = rectangle.iter().min().unwrap();
            if *min_side > max_len {
                max_len = *min_side;
                count = 1;
            } else if *min_side == max_len {
                count += 1;
            }
        }
        
        count

    }
}
