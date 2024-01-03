
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();
        let m = grid.len();
        let n = grid[0].len();
        let mut start = (0, 0);
        let mut keys = 0;
        let mut target = 0;
        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    '@' => start = (i, j),
                    'a'..='f' => keys |= 1 << (grid[i][j] as u8 - b'a'),
                    'A'..='F' => target |= 1 << (grid[i][j] as u8 - b'A'),
                    _ => {}
                }
            }
        }
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![vec![false; 64]; n]; m];
        queue.push_back((start.0, start.1, 0, 0));
        visited[start.0][start.1][0] = true;
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some((x, y, state, steps)) = queue.pop_front() {
            if state == target {
                return steps;
            }
            for (dx, dy) in &dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let c = grid[nx][ny];
                    if c == '#' {
                        continue;
                    }
                    let mut nstate = state;
                    if c.is_ascii_lowercase() {
                        nstate |= 1 << (c as u8 - b'a');
                    }
                    if c.is_ascii_uppercase() && (state & (1 << (c as u8 - b'A'))) == 0 {
                        continue;
                    }
                    if !visited[nx][ny][nstate] {
                        visited[nx][ny][nstate] = true;
                        queue.push_back((nx, ny, nstate, steps + 1));
                    }
                }
            }
        }
        -1

    }
}
