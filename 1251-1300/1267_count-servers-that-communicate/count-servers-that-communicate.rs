
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_servers = vec![0; m];
        let mut col_servers = vec![0; n];
        
        // 统计每行和每列的服务器数量

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    row_servers[i] += 1;
                    col_servers[j] += 1;
                }
            }
        }
        
        let mut count = 0;
        
        // 再次遍历矩阵，计算与其他服务器通信的服务器数量

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && (row_servers[i] > 1 || col_servers[j] > 1) {
                    count += 1;
                }
            }
        }
        
        count

    }
}
