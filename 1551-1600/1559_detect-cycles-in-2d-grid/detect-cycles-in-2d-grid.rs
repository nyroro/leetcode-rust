
impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        fn dfs(
            grid: &Vec<Vec<char>>,
            visited: &mut Vec<Vec<bool>>,
            directions: &Vec<(i32, i32)>,
            x: usize,
            y: usize,
            px: i32,
            py: i32,
        ) -> bool {
            visited[x][y] = true;
            for &(dx, dy) in directions.iter() {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == grid[x][y] {
                    if !(nx as i32 == px && ny as i32 == py) {
                        if visited[nx][ny] || dfs(grid, visited, directions, nx, ny, x as i32, y as i32) {
                            return true;
                        }
                    }
                }
            }
            false

        }

        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] {
                    if dfs(&grid, &mut visited, &directions, i, j, -1, -1) {
                        return true;
                    }
                }
            }
        }
        false

    }
}
