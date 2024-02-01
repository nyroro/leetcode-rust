
impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        
        for r in 0..rows {
            for c in 0..cols {
                let distance = (r - r_center).abs() + (c - c_center).abs();
                result.push(vec![r, c, distance]);
            }
        }
        
        result.sort_by_key(|coord| coord[2].clone());
        
        result.iter().map(|coord| vec![coord[0], coord[1]]).collect()
    }
}
