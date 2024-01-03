
impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];

        // 标记边界上的陆地单元格及其相邻的陆地单元格为已访问

        for i in 0..m {
            Self::dfs(&grid, &mut visited, i, 0);
            Self::dfs(&grid, &mut visited, i, n - 1);
        }
        for j in 0..n {
            Self::dfs(&grid, &mut visited, 0, j);
            Self::dfs(&grid, &mut visited, m - 1, j);
        }

        // 统计未被标记为已访问的陆地单元格的数量

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && !visited[i][j] {
                    count += 1;
                }
            }
        }

        count

    }

    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, row: usize, col: usize) {
        let m = grid.len();
        let n = grid[0].len();

        if row >= m || col >= n || visited[row][col] || grid[row][col] == 0 {
            return;
        }

        visited[row][col] = true;

        Self::dfs(grid, visited, row + 1, col);
        Self::dfs(grid, visited, row - 1, col);
        Self::dfs(grid, visited, row, col + 1);
        Self::dfs(grid, visited, row, col - 1);
    }
}
