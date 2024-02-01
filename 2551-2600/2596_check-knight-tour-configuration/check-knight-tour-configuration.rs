
impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len() as i32;
        let mut visited = vec![vec![false; n as usize]; n as usize];
        let moves = vec![(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)];

        fn is_valid_move(x: i32, y: i32, n: i32, visited: &Vec<Vec<bool>>) -> bool {
            x >= 0 && x < n && y >= 0 && y < n && !visited[x as usize][y as usize]
        }

        fn dfs(x: i32, y: i32, count: i32, n: i32, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, moves: &Vec<(i32, i32)>) -> bool {
            if count == n * n {
                return true;
            }

            for (dx, dy) in moves.iter() {
                let nx = x + dx;
                let ny = y + dy;
                if is_valid_move(nx, ny, n, visited) && grid[nx as usize][ny as usize] == count {
                    visited[nx as usize][ny as usize] = true;
                    if dfs(nx, ny, count + 1, n, grid, visited, moves) {
                        return true;
                    }
                    visited[nx as usize][ny as usize] = false;
                }
            }

            false

        }

        visited[0][0] = true;
        dfs(0, 0, 1, n, &grid, &mut visited, &moves)
    }
}
