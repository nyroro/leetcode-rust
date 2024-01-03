
impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        
        // Initialize diff matrix with zeros

        let mut diff = vec![vec![0; n]; m];
        
        // Arrays to store ones and zeros count for each row and column

        let mut ones_row = vec![0; m];
        let mut zeros_row = vec![0; m];
        let mut ones_col = vec![0; n];
        let mut zeros_col = vec![0; n];
        
        // Calculate ones and zeros count for each row and column

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    ones_row[i] += 1;
                    ones_col[j] += 1;
                } else {
                    zeros_row[i] += 1;
                    zeros_col[j] += 1;
                }
            }
        }
        
        // Calculate diff matrix using the given formula

        for i in 0..m {
            for j in 0..n {
                diff[i][j] = ones_row[i] + ones_col[j] - zeros_row[i] - zeros_col[j];
            }
        }
        
        diff

    }
}
