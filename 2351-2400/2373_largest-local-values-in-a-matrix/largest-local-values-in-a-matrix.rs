
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut max_local = vec![vec![0; n - 2]; n - 2];

        for i in 1..n-1 {
            for j in 1..n-1 {
                let mut max_val = std::i32::MIN;
                for x in i-1..i+2 {
                    for y in j-1..j+2 {
                        max_val = max_val.max(grid[x][y]);
                    }
                }
                max_local[i-1][j-1] = max_val;
            }
        }

        max_local

    }
}
