


impl Solution {
    pub fn string_count(n: i32) -> i64 {
        let mut ans = vec![vec![vec![0i64; 2]; 3]; 2];
        let mod_val: i64 = 1000000007;

        ans[0][0][0] = 1;

        for i in 0..n {
            let mut next = vec![vec![vec![0i64; 2]; 3]; 2];
            for j in 0..2 {
                for k in 0..3 {
                    for l in 0..2 {
                        next[j][k][l] = 0;
                    }
                }
            }
            for j in 0..2 {
                for k in 0..3 {
                    for l in 0..2 {
                        next[j][k][l] = (next[j][k][l] + ans[j][k][l] * 23) % mod_val;
                        next[1.min(j + 1)][k][l] = (next[1.min(j + 1)][k][l] + ans[j][k][l]) % mod_val;
                        next[j][2.min(k + 1)][l] = (next[j][2.min(k + 1)][l] + ans[j][k][l]) % mod_val;
                        next[j][k][1.min(l + 1)] = (next[j][k][1.min(l + 1)] + ans[j][k][l]) % mod_val;
                    }
                }
            }
            ans = next;
        }

        ans[1][2][1]
    }
}
