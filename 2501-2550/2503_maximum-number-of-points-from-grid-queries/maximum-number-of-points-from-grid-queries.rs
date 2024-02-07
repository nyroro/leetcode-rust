
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut threshold = vec![vec![0; n]; m];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((grid[0][0], 0, 0)));

        while let Some(Reverse((cost, x, y))) = heap.pop() {
            while !heap.is_empty() && threshold[x][y] > 0 {
                if let Some(Reverse((c, xx, yy))) = heap.pop() {
                    if threshold[xx][yy] == 0 {
                        heap.push(Reverse((c, xx, yy)));
                        break;
                    }
                }
            }
            if threshold[x][y] == 0 {
                threshold[x][y] = cost;
                for (nx, ny) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                    if *nx >= 0 && *nx < m.try_into().unwrap() && *ny >= 0 && *ny < n.try_into().unwrap() {
                        heap.push(Reverse((std::cmp::max(grid[*nx as usize][*ny as usize], cost), *nx, *ny)));
                    }
                }
            }
        }

        let mut elements = Vec::new();
        for i in 0..m {
            for j in 0..n {
                elements.push(threshold[i][j]);
            }
        }
        elements.sort();

        let mut result = Vec::new();
        for query in &queries {
            result.push(elements.iter().filter(|&&x| x < *query).count() as i32);
        }
        result

    }
}
