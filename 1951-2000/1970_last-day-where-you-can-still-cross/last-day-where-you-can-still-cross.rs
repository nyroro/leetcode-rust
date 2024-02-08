
use std::collections::VecDeque;



impl Solution {
    // Define the function to check if it's possible to cross on a specific day

    fn solve(row: i32, col: i32, cells: &Vec<Vec<i32>>, mid: i32) -> bool {
        // Create a 2D array to represent the matrix

        let mut arr = vec![vec![0; col as usize]; row as usize];

        // Mark the cells as flooded up to the mid day

        for i in 0..mid {
            let r = cells[i as usize][0] - 1;
            let c = cells[i as usize][1] - 1;
            arr[r as usize][c as usize] = 1;
        }

        // Use a queue to perform BFS to check if it's possible to cross

        let mut q = VecDeque::new();
        for i in 0..col {
            if arr[0][i as usize] == 0 {
                q.push_back((0, i));
                arr[0][i as usize] = 1;
            }
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((x, y)) = q.pop_front() {
            if x == row - 1 {
                return true;
            }
            for dir in &directions {
                let xx = x + dir.0;
                let yy = y + dir.1;
                if xx >= 0 && yy >= 0 && xx < row && yy < col && arr[xx as usize][yy as usize] == 0 {
                    arr[xx as usize][yy as usize] = 1;
                    q.push_back((xx, yy));
                }
            }
        }

        false

    }

    // Define the main function to find the last day

    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut i = 1;
        let mut j = cells.len() as i32;
        let mut ans = 0;

        while i <= j {
            let m = (i + j) / 2;
            if Solution::solve(row, col, &cells, m) {
                ans = m;
                i = m + 1;
            } else {
                j = m - 1;
            }
        }

        ans

    }
}
