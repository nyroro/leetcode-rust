
impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let m = seats.len();
        let n = seats[0].len();
        let mut dp = vec![vec![-1; 1 << n]; m + 1];
        dp[0][0] = 0;

        for i in 1..=m {
            let mut valid_masks = Vec::new();
            'outer: for mask in 0..1 << n {
                let mut cur = mask;
                while cur > 0 {
                    if (cur & 0b11) == 0b11 || (cur & 0b101010) != cur {
                        continue 'outer;
                    }
                    cur >>= 1;
                }
                valid_masks.push(mask);
            }

            for &mask in &valid_masks {
                for &prev_mask in &valid_masks {
                    if (mask & prev_mask) == 0 && (mask & (prev_mask << 1)) == 0 && ((mask << 1) & prev_mask) == 0 {
                        for j in 0..n {
                            if (mask >> j) & 1 == 1 && seats[i - 1][j] == '#' {
                                continue;
                            }
                        }
                        dp[i][mask] = dp[i][mask].max(dp[i - 1][prev_mask] + (mask.count_ones() as i32));
                    }
                }
            }
        }

        *dp[m].iter().max().unwrap()
    }
}
