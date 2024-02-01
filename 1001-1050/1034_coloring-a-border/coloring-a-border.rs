
impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = grid.clone();

        let start_color = grid[row as usize][col as usize];
        let mut visited = vec![vec![false; n]; m];

        fn dfs(
            grid: &Vec<Vec<i32>>,
            result: &mut Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            color: i32,
            start_color: i32,
            row: usize,
            col: usize,
            m: usize,
            n: usize,
        ) {
            if row >= m || col >= n || row == 0 || col == 0 || grid[row][col] != start_color {
                return;
            }
            if visited[row][col] {
                return;
            }
            visited[row][col] = true;

            if row == 0 || col == 0 || row == m - 1 || col == n - 1

                || grid[row - 1][col] != start_color

                || grid[row + 1][col] != start_color

                || grid[row][col - 1] != start_color

                || grid[row][col + 1] != start_color

            {
                result[row][col] = color;
            }

            dfs(grid, result, visited, color, start_color, row + 1, col, m, n);
            dfs(grid, result, visited, color, start_color, row - 1, col, m, n);
            dfs(grid, result, visited, color, start_color, row, col + 1, m, n);
            dfs(grid, result, visited, color, start_color, row, col - 1, m, n);
        }

        dfs(
            &grid,
            &mut result,
            &mut visited,
            color,
            start_color,
            row as usize,
            col as usize,
            m,
            n,
        );

        result

    }
}
