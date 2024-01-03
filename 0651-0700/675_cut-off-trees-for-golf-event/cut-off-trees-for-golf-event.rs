
impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let m = forest.len();
        let n = forest[0].len();
        let mut trees: Vec<(usize, usize, i32)> = Vec::new();
        
        // 将树按照高度从低到高排序，并记录位置和高度

        for i in 0..m {
            for j in 0..n {
                if forest[i][j] > 1 {
                    trees.push((i, j, forest[i][j]));
                }
            }
        }
        trees.sort_by_key(|(_, _, h)| *h);
        
        let mut steps = 0;
        let mut cur_pos = (0, 0);
        
        // 遍历每棵树，计算到达目标树的最短路径长度

        for (i, j, _) in trees {
            let dist = Self::bfs(&forest, cur_pos, (i, j));
            if dist == -1 {
                return -1;
            }
            steps += dist;
            cur_pos = (i, j);
        }
        
        steps

    }
    
    // 使用BFS算法计算从起点到目标位置的最短路径长度

    fn bfs(forest: &Vec<Vec<i32>>, start: (usize, usize), target: (usize, usize)) -> i32 {
        let m = forest.len();
        let n = forest[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut queue = std::collections::VecDeque::new();
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        queue.push_back((start, 0));
        visited[start.0][start.1] = true;
        
        while let Some((pos, steps)) = queue.pop_front() {
            if pos == target {
                return steps;
            }
            
            for (dx, dy) in &dirs {
                let nx = pos.0 as i32 + dx;
                let ny = pos.1 as i32 + dy;
                
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && !visited[nx as usize][ny as usize] && forest[nx as usize][ny as usize] != 0 {
                    queue.push_back(((nx as usize, ny as usize), steps + 1));
                    visited[nx as usize][ny as usize] = true;
                }
            }
        }
        
        -1

    }
}
