
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start = mat.clone(); // 使用 clone 方法创建副本

        queue.push_back((start.clone(), 0)); // 使用 clone 方法创建副本

        visited.insert(start);

        while let Some((cur, steps)) = queue.pop_front() {
            if cur.iter().all(|row| row.iter().all(|&x| x == 0)) {
                return steps;
            }
            for i in 0..m {
                for j in 0..n {
                    let mut next = cur.clone();
                    next[i][j] = 1 - next[i][j];
                    if i > 0 {
                        next[i - 1][j] = 1 - next[i - 1][j];
                    }
                    if i < m - 1 {
                        next[i + 1][j] = 1 - next[i + 1][j];
                    }
                    if j > 0 {
                        next[i][j - 1] = 1 - next[i][j - 1];
                    }
                    if j < n - 1 {
                        next[i][j + 1] = 1 - next[i][j + 1];
                    }
                    if !visited.contains(&next) {
                        visited.insert(next.clone());
                        queue.push_back((next, steps + 1));
                    }
                }
            }
        }
        -1

    }
}
