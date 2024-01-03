
use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len();
        let n = maze[0].len();
        let (start_x, start_y) = (entrance[0] as usize, entrance[1] as usize);
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; n]; m];
        let directions = vec![-1, 0, 1, 0, -1];
        let mut steps = 0;
        
        queue.push_back((start_x, start_y));
        visited[start_x][start_y] = true;
        
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let (x, y) = queue.pop_front().unwrap();
                if (x == 0 || x == m - 1 || y == 0 || y == n - 1) && (x != start_x || y != start_y) {
                    return steps;
                }
                for k in 0..4 {
                    let (new_x, new_y) = (x as i32 + directions[k], y as i32 + directions[k + 1]);
                    if new_x >= 0 && new_x < m as i32 && new_y >= 0 && new_y < n as i32 && maze[new_x as usize][new_y as usize] == '.' && !visited[new_x as usize][new_y as usize] {
                        queue.push_back((new_x as usize, new_y as usize));
                        visited[new_x as usize][new_y as usize] = true;
                    }
                }
            }
            steps += 1;
        }
        
        -1

    }
}
