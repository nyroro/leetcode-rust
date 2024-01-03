
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut lucky_nums: Vec<i32> = Vec::new();

        for row in &matrix {
            let min_row_val = *row.iter().min().unwrap();
            let min_row_index = row.iter().position(|&x| x == min_row_val).unwrap();

            let mut is_max_in_col = true;
            for i in 0..matrix.len() {
                if matrix[i][min_row_index] > min_row_val {
                    is_max_in_col = false;
                    break;
                }
            }

            if is_max_in_col {
                lucky_nums.push(min_row_val);
            }
        }

        lucky_nums

    }
}
