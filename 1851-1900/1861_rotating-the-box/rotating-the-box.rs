
impl Solution {
    pub fn rotate_the_box(input_box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = input_box.len();
        let n = input_box[0].len();
        let mut result = vec![vec!['.'; m]; n];

        for i in 0..m {
            let mut k = n - 1;
            for j in (0..n).rev() {
                match input_box[i][j] {
                    '#' => {
                        result[k][m - 1 - i] = '#';
                        k -= 1;
                    }
                    '*' => {
                        result[j][m - 1 - i] = '*';
                        k = j - 1;
                    }
                    _ => {}
                }
            }
        }

        result

    }
}
