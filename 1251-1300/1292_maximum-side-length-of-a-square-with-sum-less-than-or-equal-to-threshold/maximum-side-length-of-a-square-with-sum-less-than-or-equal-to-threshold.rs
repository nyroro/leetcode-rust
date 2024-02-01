
use std::convert::TryInto;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut prefix_sum = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                prefix_sum[i][j] = mat[i - 1][j - 1] + prefix_sum[i - 1][j] + prefix_sum[i][j - 1] - prefix_sum[i - 1][j - 1];
            }
        }
        let mut max_side_length = 0;
        for i in 1..=m {
            for j in 1..=n {
                for k in (max_side_length + 1..=std::cmp::min(m - i + 1, n - j + 1)).rev() {
                    let sum = prefix_sum[i + k - 1][j + k - 1] - prefix_sum[i + k - 1][j - 1] - prefix_sum[i - 1][j + k - 1] + prefix_sum[i - 1][j - 1];
                    if sum <= threshold {
                        max_side_length = k;
                        break;
                    }
                }
            }
        }
        max_side_length.try_into().unwrap()
    }
}
