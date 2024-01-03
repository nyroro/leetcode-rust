
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        
        for row in image {
            let mut flipped_row: Vec<i32> = Vec::new();
            
            for &num in row.iter().rev() {
                flipped_row.push(1 - num);
            }
            
            result.push(flipped_row);
        }
        
        result

    }
}
