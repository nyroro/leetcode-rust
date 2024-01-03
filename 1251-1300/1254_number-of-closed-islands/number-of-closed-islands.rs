
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut closed_islands = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 0 && (i == 0 || i == rows - 1 || j == 0 || j == cols - 1) {
                    Solution::dfs(&mut grid, i as i32, j as i32);
                }
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 0 {
                    Solution::dfs(&mut grid, i as i32, j as i32);
                    closed_islands += 1;
                }
            }
        }

        closed_islands

    }

    fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32) {
        if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[0].len() as i32 || grid[r as usize][c as usize] == 1 {
            return;
        }

        grid[r as usize][c as usize] = 1;
        Solution::dfs(grid, r - 1, c);
        Solution::dfs(grid, r + 1, c);
        Solution::dfs(grid, r, c - 1);
        Solution::dfs(grid, r, c + 1);
    }
}
