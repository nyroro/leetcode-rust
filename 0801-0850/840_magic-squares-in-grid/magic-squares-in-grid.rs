
use std::collections::HashSet;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let numset: HashSet<i32> = (1..=9).collect();

        for rowix in 1..grid.len() - 1 {
            let row = &grid[rowix];
            let fiveix: Vec<usize> = (1..row.len() - 1).filter(|&ix| row[ix] == 5).collect();
            
            for &five in &fiveix {
                let mut subgrid: Vec<Vec<i32>> = Vec::new();
                for i in (rowix - 1)..=(rowix + 1) {
                    subgrid.push(grid[i][five - 1..=five + 1].to_vec());
                }

                let flatgrid: HashSet<i32> = subgrid.iter().flat_map(|row| row.iter()).cloned().collect();
                let diff: HashSet<i32> = numset.difference(&flatgrid).cloned().collect();

                let row_sum: Vec<i32> = subgrid.iter().map(|row| row.iter().sum()).collect();
                let col_sum: Vec<i32> = (0..3).map(|j| subgrid.iter().map(|row| row[j]).sum()).collect();

                if row_sum.iter().all(|&sum| sum == 15) && col_sum.iter().all(|&sum| sum == 15) && diff.is_empty() {
                    count += 1;
                }
            }
        }

        count

    }
}
