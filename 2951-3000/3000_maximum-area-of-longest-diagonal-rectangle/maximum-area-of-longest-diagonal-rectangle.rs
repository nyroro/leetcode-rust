
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_diagonal_length = 0.0;
        let mut max_area = 0;

        for dimension in &dimensions {
            let length = dimension[0] as f64;
            let width = dimension[1] as f64;
            let diagonal_length = (length * length + width * width).sqrt();

            if diagonal_length > max_diagonal_length {
                max_diagonal_length = diagonal_length;
                max_area = (length * width) as i32;
            } else if diagonal_length == max_diagonal_length {
                max_area = max_area.max((length * width) as i32);
            }
        }

        max_area

    }
}
