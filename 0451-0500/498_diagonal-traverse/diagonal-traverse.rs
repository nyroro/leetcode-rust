
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut result = Vec::new();
        let mut direction = 1;
        let mut row = 0;
        let mut col = 0;

        while result.len() < m * n {
            result.push(mat[row][col]);

            if direction == 1 {
                if col == n - 1 {
                    if row == m - 1 {
                        break;
                    } else {
                        row += 1;
                        direction = -1;
                    }
                } else if row == 0 {
                    col += 1;
                    direction = -1;
                } else {
                    row -= 1;
                    col += 1;
                }
            } else {
                if row == m - 1 {
                    if col == n - 1 {
                        break;
                    } else {
                        col += 1;
                        direction = 1;
                    }
                } else if col == 0 {
                    row += 1;
                    direction = 1;
                } else {
                    row += 1;
                    col -= 1;
                }
            }
        }

        result

    }
}
