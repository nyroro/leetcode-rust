


impl Solution {
    pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let mut dp = vec![vec![vec![vec![vec![vec![-1; 64]; 64]; 7]; 7]; 5]; 5];
        
        fn cost(i: i32, j: i32, mask_in: i32, mask_ex: i32, d: i32, m: i32) -> i32 {
            let mut ans = 0;
            if i > 0 && (mask_in & (1 << (m - 1))) != 0 {
                ans += d - 30;
            }
            if j > 0 && (mask_in & 1) != 0 {
                ans += d - 30;
            }
            if i > 0 && (mask_ex & (1 << (m - 1))) != 0 {
                ans += d + 20;
            }
            if j > 0 && (mask_ex & 1) != 0 {
                ans += d + 20;
            }
            ans

        }

        fn f(i: i32, j: i32, mask_ex: i32, mask_in: i32, ic: i32, ec: i32, n: i32, m: i32, dp: &mut Vec<Vec<Vec<Vec<Vec<Vec<i32>>>>>>) -> i32 {
            if i == n {
                return 0;
            }
            if dp[i as usize][j as usize][ic as usize][ec as usize][mask_ex as usize][mask_in as usize] != -1 {
                return dp[i as usize][j as usize][ic as usize][ec as usize][mask_ex as usize][mask_in as usize];
            }
            let n_mask_ex = (mask_ex << 1) & 63;
            let n_mask_in = (mask_in << 1) & 63;
            let mut ans;
            if j + 1 < m {
                ans = f(i, j + 1, n_mask_ex, n_mask_in, ic, ec, n, m, dp);
            } else {
                ans = f(i + 1, 0, n_mask_ex, n_mask_in, ic, ec, n, m, dp);
            }
            if ic > 0 {
                let ct = cost(i, j, mask_in, mask_ex, -30, m);
                if j + 1 < m {
                    ans = ans.max(120 + ct + f(i, j + 1, n_mask_ex, n_mask_in + 1, ic - 1, ec, n, m, dp));
                } else {
                    ans = ans.max(120 + ct + f(i + 1, 0, n_mask_ex, n_mask_in + 1, ic - 1, ec, n, m, dp));
                }
            }
            if ec > 0 {
                let ct = cost(i, j, mask_in, mask_ex, 20, m);
                if j + 1 < m {
                    ans = ans.max(40 + ct + f(i, j + 1, n_mask_ex + 1, n_mask_in, ic, ec - 1, n, m, dp));
                } else {
                    ans = ans.max(40 + ct + f(i + 1, 0, n_mask_ex + 1, n_mask_in, ic, ec - 1, n, m, dp));
                }
            }
            dp[i as usize][j as usize][ic as usize][ec as usize][mask_ex as usize][mask_in as usize] = ans;
            ans

        }

        f(0, 0, 0, 0, introverts_count, extroverts_count, m, n, &mut dp)
    }
}
