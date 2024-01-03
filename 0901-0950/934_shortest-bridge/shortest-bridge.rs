
impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut queue = Vec::new();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        // DFS to find the first island and mark it as 2

        let mut found = false;
        for i in 0..n {
            if found {
                break;
            }
            for j in 0..n {
                if grid[i][j] == 1 {
                    Self::dfs(&grid, i, j, &mut visited, &mut queue);
                    found = true;
                    break;
                }
            }
        }

        // BFS to find the second island and calculate the minimum steps

        let mut steps = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let (x, y) = queue.remove(0);
                for (dx, dy) in &directions {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if !visited[nx][ny] {
                            if grid[nx][ny] == 1 {
                                return steps;
                            }
                            visited[nx][ny] = true;
                            queue.push((nx, ny));
                        }
                    }
                }
            }
            steps += 1;
        }

        -1 // If no solution is found

    }

    fn dfs(grid: &Vec<Vec<i32>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, queue: &mut Vec<(usize, usize)>) {
        let n = grid.len();
        if x >= 0 && x < n && y >= 0 && y < n && !visited[x][y] && grid[x][y] == 1 {
            visited[x][y] = true;
            queue.push((x, y));
            Self::dfs(grid, x + 1, y, visited, queue);
            Self::dfs(grid, x - 1, y, visited, queue);
            Self::dfs(grid, x, y + 1, visited, queue);
            Self::dfs(grid, x, y - 1, visited, queue);
        }
    }
}
