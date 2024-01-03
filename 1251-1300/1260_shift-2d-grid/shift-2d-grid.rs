
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut shifted_grid = vec![vec![0; n]; m];
        
        for i in 0..m {
            for j in 0..n {
                let new_pos = (i * n + j + k as usize) % (m * n);
                let new_i = new_pos / n;
                let new_j = new_pos % n;
                shifted_grid[new_i][new_j] = grid[i][j];
            }
        }
        
        shifted_grid

    }
}
