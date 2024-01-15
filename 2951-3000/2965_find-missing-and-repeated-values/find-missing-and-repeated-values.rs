
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut count = vec![0; n * n];
        let mut result = vec![0; 2];

        for i in 0..n {
            for j in 0..n {
                let num = grid[i][j] as usize;
                count[num - 1] += 1;
            }
        }

        for (i, &c) in count.iter().enumerate() {
            if c == 2 {
                result[0] = (i + 1) as i32;
            } else if c == 0 {
                result[1] = (i + 1) as i32;
            }
        }

        result

    }
}
