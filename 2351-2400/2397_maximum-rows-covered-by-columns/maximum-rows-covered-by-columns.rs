
impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut max_rows_covered = 0;
        
        fn backtrack(
            matrix: &Vec<Vec<i32>>,
            num_select: i32,
            start_col: usize,
            selected_cols: &mut Vec<usize>,
            max_rows_covered: &mut i32,
        ) {
            if selected_cols.len() == num_select as usize {
                let mut rows_covered = 0;
                for row in matrix {
                    let mut covered = true;
                    for (i, &cell) in row.iter().enumerate() {
                        if cell == 1 && !selected_cols.contains(&i) {
                            covered = false;
                            break;
                        }
                    }
                    if covered {
                        rows_covered += 1;
                    }
                }
                *max_rows_covered = (*max_rows_covered).max(rows_covered);
                return;
            }
            
            for i in start_col..matrix[0].len() {
                selected_cols.push(i);
                backtrack(matrix, num_select, i + 1, selected_cols, max_rows_covered);
                selected_cols.pop();
            }
        }
        
        let mut selected_cols = Vec::new();
        backtrack(&matrix, num_select, 0, &mut selected_cols, &mut max_rows_covered);
        
        max_rows_covered

    }
}
