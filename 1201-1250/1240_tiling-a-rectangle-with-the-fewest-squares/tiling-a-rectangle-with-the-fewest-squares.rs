


impl Solution {
    fn is_available(dp: &Vec<Vec<bool>>, r: usize, c: usize, k: usize) -> bool {
        let (r_end, c_end) = (r + k, c + k);
        for r1 in r..r_end {
            for c1 in c..c_end {
                if dp[r1][c1] {
                    return false;
                }
            }
        }
        true

    }

    fn fill_up(dp: &mut Vec<Vec<bool>>, r: usize, c: usize, k: usize, val: bool) {
        let (r_end, c_end) = (r + k, c + k);
        for r1 in r..r_end {
            for c1 in c..c_end {
                dp[r1][c1] = val;
            }
        }
    }

    fn compute_res(r: usize, mut c: usize, res: &mut i32, cur_res: i32, dp: &mut Vec<Vec<bool>>) {
        let (rows, cols) = (dp.len(), dp[0].len());
        if cur_res >= *res {
            return;
        }
        if c >= cols {
            Self::compute_res(r + 1, 0, res, cur_res, dp);
            return;
        }
        if r == rows {
            *res = (*res).min(cur_res);
            return;
        }
        if dp[r][c] {
            while c < cols && dp[r][c] {
                c += 1;
            }
            Self::compute_res(r, c, res, cur_res, dp);
            return;
        }
        for k in (0..(rows - r).min(cols - c)).rev() {
            if !Self::is_available(dp, r, c, k + 1) {
                break;
            }
            Self::fill_up(dp, r, c, k + 1, true);
            Self::compute_res(r, c + k + 1, res, cur_res + 1, dp);
            Self::fill_up(dp, r, c, k + 1, false);
        }
    }

    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let (n, m) = (n as usize, m as usize);
        let mut res = i32::MAX;
        let mut cur_res = 0;
        let mut dp = vec![vec![false; m]; n];
        Self::compute_res(0, 0, &mut res, cur_res, &mut dp);
        res

    }
}
