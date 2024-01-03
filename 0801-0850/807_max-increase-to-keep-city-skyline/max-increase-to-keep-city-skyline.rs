
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut row_max = vec![0; n];
        let mut col_max = vec![0; n];
        
        // 计算每行和每列的天际线高度

        for i in 0..n {
            for j in 0..n {
                row_max[i] = row_max[i].max(grid[i][j]);
                col_max[j] = col_max[j].max(grid[i][j]);
            }
        }
        
        let mut max_increase = 0;
        
        // 计算可以增加的最大高度

        for i in 0..n {
            for j in 0..n {
                max_increase += (row_max[i].min(col_max[j]) - grid[i][j]).max(0);
            }
        }
        
        max_increase

    }
}
