
impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let m = seats.len();
        let n = seats[0].len();
        let mut dp = vec![vec![-1; 1 << n]; m + 1];
        dp[0][0] = 0;

        for i in 1..=m {
            for mask in 0..1 << n {
                if (mask & (mask << 1)) == 0 && (mask & (mask >> 1)) == 0 {
                    let mut valid = true;
                    for j in 0..n {
                        if (mask >> j) & 1 == 1 && seats[i - 1][j] == '#' {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        for prev_mask in 0..1 << n {
                            if (mask & (prev_mask << 1)) == 0 && ((mask << 1) & prev_mask) == 0 {
                                dp[i][mask] = dp[i][mask].max(dp[i - 1][prev_mask] + (mask.count_ones() as i32));
                            }
                        }
                    }
                }
            }
        }

        *dp[m].iter().max().unwrap()
    }
}
