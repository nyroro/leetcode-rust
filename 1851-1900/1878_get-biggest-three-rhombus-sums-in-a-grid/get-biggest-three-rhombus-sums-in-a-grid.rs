
impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::BTreeSet;

        let rows = grid.len();
        let columns = grid[0].len();
        let max_side = (std::cmp::min(rows, columns) + 1) / 2;
        let mut set = BTreeSet::new();

        for i in 0..rows {
            for j in 0..columns {
                set.insert(grid[i][j]);
                if set.len() > 3 {
                    set.remove(&set.iter().next().unwrap().clone());
                }
            }
        }

        for side in 2..=max_side {
            let row_start = 0;
            let row_end = rows - (side * 2 - 1);
            let column_start = side - 1;
            let column_end = columns - side;

            for i in row_start..=row_end {
                for j in column_start..=column_end {
                    let sum = Self::get_sum(&grid, i, j, side);
                    set.insert(sum);
                    if set.len() > 3 {
                        set.remove(&set.iter().next().unwrap().clone());
                    }
                }
            }
        }

        let mut biggest = set.into_iter().collect::<Vec<i32>>();
        biggest.reverse();
        biggest

    }

    fn get_sum(grid: &Vec<Vec<i32>>, top_row: usize, top_column: usize, side: usize) -> i32 {
        let bottom_row = top_row + (side - 1) * 2;
        let mut sum = grid[top_row][top_column] + grid[bottom_row][top_column];

        for i in 1..side {
            sum += grid[top_row + i][top_column - i] + grid[top_row + i][top_column + i];
        }

        for i in 1..(side - 1) {
            sum += grid[bottom_row - i][top_column - i] + grid[bottom_row - i][top_column + i];
        }

        sum

    }
}
