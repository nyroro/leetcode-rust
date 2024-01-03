
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut row = r_start;
        let mut col = c_start;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut direction = 0;
        let mut step = 1;
        let total_steps = rows * cols;
        
        while result.len() < total_steps as usize {
            for _ in 0..step {
                if row >= 0 && row < rows && col >= 0 && col < cols {
                    result.push(vec![row, col]);
                }
                row += directions[direction].0;
                col += directions[direction].1;
            }
            
            direction = (direction + 1) % 4;
            
            if direction % 2 == 0 {
                step += 1;
            }
        }
        
        result

    }
}
