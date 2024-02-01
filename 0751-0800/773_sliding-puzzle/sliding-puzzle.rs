
use std::collections::{HashSet, VecDeque, HashMap};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let target = vec![vec![1, 2, 3], vec![4, 5, 0]];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut steps = HashMap::new();

        queue.push_back((board.clone(), 0));
        visited.insert(board.clone());
        steps.insert(board.clone(), 0);

        while let Some((state, step)) = queue.pop_front() {
            if state == target {
                return step;
            }

            let (i, j) = find_zero(&state);

            for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni >= 0 && ni < 2 && nj >= 0 && nj < 3 {
                    let mut new_state = state.clone();
                    new_state[i][j] = new_state[ni as usize][nj as usize];
                    new_state[ni as usize][nj as usize] = 0;

                    if !visited.contains(&new_state) {
                        queue.push_back((new_state.clone(), step + 1));
                        visited.insert(new_state.clone());
                        steps.insert(new_state, step + 1);
                    }
                }
            }
        }

        -1

    }
}

fn find_zero(board: &Vec<Vec<i32>>) -> (usize, usize) {
    for i in 0..2 {
        for j in 0..3 {
            if board[i][j] == 0 {
                return (i, j);
            }
        }
    }
    unreachable!()
}
