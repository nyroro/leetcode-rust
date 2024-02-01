
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    distance: i32,
    price: i32,
    row: usize,
    col: usize,
}

impl Solution {
    pub fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut queue = Vec::new();
        let mut distance = vec![vec![-1; n]; m];
        let mut directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut items = BinaryHeap::new();

        distance[start[0] as usize][start[1] as usize] = 0;
        queue.push((start[0] as usize, start[1] as usize));

        while let Some((x, y)) = queue.pop() {
            let current_distance = distance[x][y];
            let val = grid[x][y];
            if pricing[0] <= val && val <= pricing[1] {
                items.push(Reverse(Item { distance: current_distance, price: val, row: x, col: y }));
            }
            for (dx, dy) in &directions {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                if new_x < m && new_y < n && distance[new_x][new_y] == -1 && grid[new_x][new_y] != 0 {
                    distance[new_x][new_y] = current_distance + 1;
                    queue.push((new_x, new_y));
                }
            }
        }

        let mut result = Vec::new();
        for _ in 0..k {
            if let Some(Reverse(item)) = items.pop() {
                result.push(vec![item.row as i32, item.col as i32]);
            } else {
                break;
            }
        }
        result

    }
}
