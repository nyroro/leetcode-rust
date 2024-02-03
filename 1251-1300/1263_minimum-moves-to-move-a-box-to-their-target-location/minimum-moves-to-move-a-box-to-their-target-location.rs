
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pushes: i32,
    player: (usize, usize),
    box_pos: (usize, usize),
}

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut player = (0, 0);
        let mut box_pos = (0, 0);
        let mut target = (0, 0);

        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    'S' => player = (i, j),
                    'B' => box_pos = (i, j),
                    'T' => target = (i, j),
                    _ => {}
                }
            }
        }

        let mut seen = HashSet::new();
        let mut queue = BinaryHeap::new();

        queue.push(Reverse((0, player, box_pos)));
        seen.insert((player, box_pos));

        while let Some(Reverse((pushes, player, box_pos))) = queue.pop() {
            if box_pos == target {
                return pushes;
            }

            let (pi, pj) = player;
            let (bi, bj) = box_pos;

            for &(di, dj) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let ni = (pi as i32 + di) as usize;
                let nj = (pj as i32 + dj) as usize;

                if ni < m && nj < n {
                    if (ni, nj) == (bi, bj) {
                        let nbi = (bi as i32 + di) as usize;
                        let nbj = (bj as i32 + dj) as usize;

                        if nbi < m && nbj < n && grid[nbi][nbj] != '#' && !seen.contains(&(player, (nbi, nbj))) {
                            queue.push(Reverse((pushes + 1, player, (nbi, nbj))));
                            seen.insert((player, (nbi, nbj)));
                        }
                    } else if grid[ni][nj] != '#' && !seen.contains(&((ni, nj), box_pos)) {
                        queue.push(Reverse((pushes, (ni, nj), box_pos)));
                        seen.insert(((ni, nj), box_pos));
                    }
                }
            }
        }

        -1

    }
}
