
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dist = vec![vec![std::i32::MAX; n]; n];
        let mut min_heap = std::collections::BinaryHeap::new();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        dist[0][0] = grid[0][0];
        min_heap.push((std::cmp::Reverse(grid[0][0]), 0, 0));

        while let Some((std::cmp::Reverse(d), x, y)) = min_heap.pop() {
            if x == n - 1 && y == n - 1 {
                return d;
            }
            for (dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let nd = std::cmp::max(d, grid[nx][ny]);
                    if nd < dist[nx][ny] {
                        dist[nx][ny] = nd;
                        min_heap.push((std::cmp::Reverse(nd), nx, ny));
                    }
                }
            }
        }
        -1

    }
}
