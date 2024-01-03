
use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut visited = vec![false; n * n + 1];
        let mut queue = VecDeque::new();
        queue.push_back(1);
        visited[1] = true;
        let mut steps = 0;
        
        while !queue.is_empty() {
            let size = queue.len();
            
            for _ in 0..size {
                let curr = queue.pop_front().unwrap();
                
                if curr == n * n {
                    return steps;
                }
                
                for i in 1..=6 {
                    let next = curr + i;
                    
                    if next > n * n {
                        break;
                    }
                    
                    let (row, col) = get_coordinates(next, n);
                    let dest = if board[row][col] == -1 { next } else { board[row][col] as usize };
                    
                    if !visited[dest] {
                        visited[dest] = true;
                        queue.push_back(dest);
                    }
                }
            }
            
            steps += 1;
        }
        
        -1

    }
}

fn get_coordinates(num: usize, n: usize) -> (usize, usize) {
    let row = (num - 1) / n;
    let col = if row % 2 == 0 { (num - 1) % n } else { n - 1 - (num - 1) % n };
    (n - 1 - row, col)
}
