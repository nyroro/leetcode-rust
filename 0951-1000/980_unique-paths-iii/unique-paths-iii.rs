


impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut empty_count = 0;
        let mut start = (0, 0);
        let mut end = (0, 0);

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    empty_count += 1;
                } else if grid[i][j] == 1 {
                    start = (i as i32, j as i32);
                } else if grid[i][j] == 2 {
                    end = (i as i32, j as i32);
                }
            }
        }

        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        Solution::dfs(&grid, &mut visited, &mut count, &start, &end, empty_count);

        count

    }

    fn dfs(
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        count: &mut i32,
        cur: &(i32, i32),
        end: &(i32, i32),
        empty_count: i32,
    ) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        if *cur == *end {
            if empty_count == -1 {
                *count += 1;
            }
            return;
        }

        visited[cur.0 as usize][cur.1 as usize] = true;

        for dir in &directions {
            let new_x = cur.0 + dir.0;
            let new_y = cur.1 + dir.1;

            if new_x >= 0

                && new_x < grid.len() as i32

                && new_y >= 0

                && new_y < grid[0].len() as i32

                && !visited[new_x as usize][new_y as usize]
                && grid[new_x as usize][new_y as usize] != -1

            {
                Solution::dfs(
                    grid,
                    visited,
                    count,
                    &(new_x, new_y),
                    end,
                    empty_count - 1,
                );
            }
        }

        visited[cur.0 as usize][cur.1 as usize] = false;
    }
}
