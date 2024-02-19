
use std::collections::HashSet;

impl Solution {
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans: Vec<i32> = Vec::new();
        let mut bi: Vec<i32> = vec![0; n];

        for i in 0..n {
            let mut num = 0;
            let mut is_one = false;
            for j in 0..m {
                if grid[i][j] == 1 {
                    is_one = true;
                    num |= 1 << j;
                }
            }
            if !is_one {
                ans.push(i as i32);
                return ans;
            }
            bi[i] = num;
        }

        for i in 0..n {
            for j in (i + 1)..n {
                if bi[i] & bi[j] == 0 {
                    ans.push(i as i32);
                    ans.push(j as i32);
                    return ans;
                }
            }
        }
        ans

    }
}
