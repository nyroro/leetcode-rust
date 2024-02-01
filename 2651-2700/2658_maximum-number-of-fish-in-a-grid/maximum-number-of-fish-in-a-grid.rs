
use std::collections::VecDeque;

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_fish = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    let mut visited = vec![vec![false; n]; m];
                    max_fish = max_fish.max(Self::bfs(&grid, &mut visited, i, j));
                }
            }
        }

        max_fish

    }

    fn bfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, r: usize, c: usize) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut fish_count = 0;
        let mut queue = VecDeque::new();
        queue.push_back((r, c));

        while !queue.is_empty() {
            let (cur_r, cur_c) = queue.pop_front().unwrap();
            if cur_r < m && cur_c < n && !visited[cur_r][cur_c] && grid[cur_r][cur_c] > 0 {
                visited[cur_r][cur_c] = true;
                fish_count += grid[cur_r][cur_c];

                let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
                for (dr, dc) in directions {
                    let new_r = (cur_r as i32 + dr) as usize;
                    let new_c = (cur_c as i32 + dc) as usize;
                    queue.push_back((new_r, new_c));
                }
            }
        }

        fish_count

    }
}
