
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_fish = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    let mut visited = vec![vec![false; n]; m];
                    max_fish = max_fish.max(Self::dfs(&grid, &mut visited, i, j));
                }
            }
        }

        max_fish

    }

    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, r: usize, c: usize) -> i32 {
        if r >= grid.len() || c >= grid[0].len() || visited[r][c] || grid[r][c] == 0 {
            return 0;
        }

        visited[r][c] = true;
        let fish_count = grid[r][c];
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut max_fish = fish_count;

        for (dr, dc) in directions {
            let new_r = (r as i32 + dr) as usize;
            let new_c = (c as i32 + dc) as usize;
            max_fish = max_fish.max(Self::dfs(grid, visited, new_r, new_c));
        }

        visited[r][c] = false;
        max_fish + fish_count

    }
}
