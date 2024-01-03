
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }
        
        let mut result: Vec<i32> = Vec::new();
        let (mut row_start, mut row_end) = (0, matrix.len() as i32 - 1);
        let (mut col_start, mut col_end) = (0, matrix[0].len() as i32 - 1);
        
        while row_start <= row_end && col_start <= col_end {
            // Traverse right

            for col in col_start..=col_end {
                result.push(matrix[row_start as usize][col as usize]);
            }
            row_start += 1;
            
            // Traverse down

            for row in row_start..=row_end {
                result.push(matrix[row as usize][col_end as usize]);
            }
            col_end -= 1;
            
            // Traverse left

            if row_start <= row_end {
                for col in (col_start..=col_end).rev() {
                    result.push(matrix[row_end as usize][col as usize]);
                }
                row_end -= 1;
            }
            
            // Traverse up

            if col_start <= col_end {
                for row in (row_start..=row_end).rev() {
                    result.push(matrix[row as usize][col_start as usize]);
                }
                col_start += 1;
            }
        }
        
        result

    }
}
