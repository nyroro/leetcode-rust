
impl Solution {
    pub fn rotate_the_box(box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box.len();
        let n = box[0].len();
        let mut result = vec![vec!['.'; m]; n];

        for i in 0..m {
            let mut k = n - 1;
            for j in (0..n).rev() {
                match box[i][j] {
                    '#' => {
                        result[j][m - 1 - i] = '.';
                        result[k][m - 1 - i] = '#';
                        k -= 1;
                    }
                    '*' => {
                        k = j - 1;
                    }
                    _ => {}
                }
            }
        }

        result

    }
}
