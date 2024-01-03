
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        Self::dfs(&grid, 0, 0, &mut visited)
    }
    
    fn dfs(grid: &Vec<Vec<i32>>, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        
        // 判断是否到达终点

        if row == m - 1 && col == n - 1 {
            return true;
        }
        
        // 标记当前单元格为已访问

        visited[row][col] = true;
        
        // 获取当前单元格的街道连接规则

        let street = grid[row][col];
        
        // 尝试移动到相邻的单元格

        if street == 1 {
            if col > 0 && !visited[row][col - 1] && (grid[row][col - 1] == 1 || grid[row][col - 1] == 4 || grid[row][col - 1] == 6) {
                if Self::dfs(grid, row, col - 1, visited) {
                    return true;
                }
            }
            if col < n - 1 && !visited[row][col + 1] && (grid[row][col + 1] == 1 || grid[row][col + 1] == 3 || grid[row][col + 1] == 5) {
                if Self::dfs(grid, row, col + 1, visited) {
                    return true;
                }
            }
        } else if street == 2 {
            if row > 0 && !visited[row - 1][col] && (grid[row - 1][col] == 2 || grid[row - 1][col] == 3 || grid[row - 1][col] == 4) {
                if Self::dfs(grid, row - 1, col, visited) {
                    return true;
                }
            }
            if row < m - 1 && !visited[row + 1][col] && (grid[row + 1][col] == 2 || grid[row + 1][col] == 5 || grid[row + 1][col] == 6) {
                if Self::dfs(grid, row + 1, col, visited) {
                    return true;
                }
            }
        } else if street == 3 {
            if row < m - 1 && !visited[row + 1][col] && (grid[row + 1][col] == 2 || grid[row + 1][col] == 5 || grid[row + 1][col] == 6) {
                if Self::dfs(grid, row + 1, col, visited) {
                    return true;
                }
            }
            if col > 0 && !visited[row][col - 1] && (grid[row][col - 1] == 1 || grid[row][col - 1] == 4 || grid[row][col - 1] == 6) {
                if Self::dfs(grid, row, col - 1, visited) {
                    return true;
                }
            }
        } else if street == 4 {
            if row < m - 1 && !visited[row + 1][col] && (grid[row + 1][col] == 2 || grid[row + 1][col] == 5 || grid[row + 1][col] == 6) {
                if Self::dfs(grid, row + 1, col, visited) {
                    return true;
                }
            }
            if col < n - 1 && !visited[row][col + 1] && (grid[row][col + 1] == 1 || grid[row][col + 1] == 3 || grid[row][col + 1] == 5) {
                if Self::dfs(grid, row, col + 1, visited) {
                    return true;
                }
            }
        } else if street == 5 {
            if row > 0 && !visited[row - 1][col] && (grid[row - 1][col] == 2 || grid[row - 1][col] == 3 || grid[row - 1][col] == 4) {
                if Self::dfs(grid, row - 1, col, visited) {
                    return true;
                }
            }
            if col > 0 && !visited[row][col - 1] && (grid[row][col - 1] == 1 || grid[row][col - 1] == 4 || grid[row][col - 1] == 6) {
                if Self::dfs(grid, row, col - 1, visited) {
                    return true;
                }
            }
        } else if street == 6 {
            if row > 0 && !visited[row - 1][col] && (grid[row - 1][col] == 2 || grid[row - 1][col] == 3 || grid[row - 1][col] == 4) {
                if Self::dfs(grid, row - 1, col, visited) {
                    return true;
                }
            }
            if col < n - 1 && !visited[row][col + 1] && (grid[row][col + 1] == 1 || grid[row][col + 1] == 3 || grid[row][col + 1] == 5) {
                if Self::dfs(grid, row, col + 1, visited) {
                    return true;
                }
            }
        }
        
        false

    }
}
