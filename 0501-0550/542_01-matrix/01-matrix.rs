
use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut dist: Vec<Vec<i32>> = vec![vec![-1; n]; m];

        // 将所有的0添加到队列中，并将它们的距离设置为0

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    queue.push_back((i, j));
                    dist[i][j] = 0;
                }
            }
        }

        // 使用BFS计算到最近的0的距离

        while let Some((i, j)) = queue.pop_front() {
            let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            for (dx, dy) in directions.iter() {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;

                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 && dist[ni as usize][nj as usize] == -1 {
                    queue.push_back((ni as usize, nj as usize));
                    dist[ni as usize][nj as usize] = dist[i][j] + 1;
                }
            }
        }

        dist

    }
}
