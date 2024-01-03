
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut x_coordinates: Vec<i32> = points.iter().map(|point| point[0]).collect();
        x_coordinates.sort();
        
        let mut max_width = 0;
        for i in 1..x_coordinates.len() {
            let width = x_coordinates[i] - x_coordinates[i-1];
            if width > max_width {
                max_width = width;
            }
        }
        
        max_width

    }
}
