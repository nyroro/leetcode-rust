
impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_gold = 0;
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 0 {
                    max_gold = max_gold.max(Self::dfs(&grid, &mut visited, i, j));
                }
            }
        }

        max_gold

    }

    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> i32 {
        if x >= grid.len() || y >= grid[0].len() || visited[x][y] || grid[x][y] == 0 {
            return 0;
        }

        visited[x][y] = true;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut max_gold = 0;

        for (dx, dy) in directions {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            max_gold = max_gold.max(Self::dfs(grid, visited, new_x, new_y));
        }

        visited[x][y] = false;
        max_gold + grid[x][y]
    }
}
