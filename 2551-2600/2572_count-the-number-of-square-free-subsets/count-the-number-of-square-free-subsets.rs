
impl Solution {
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut a = Vec::new();
        let mut st = vec![0; 31];

        for &x in nums.iter() {
            let mut flag = true;
            let mut t = x;
            for i in 0..10 {
                let p = primes[i];
                if t % p == 0 {
                    let mut cnt = 0;
                    st[x as usize] |= 1 << i;
                    while t % p == 0 {
                        cnt += 1;
                        t /= p;
                    }
                    if cnt > 1 {
                        flag = false;
                        break;
                    }
                }
            }
            if flag {
                a.push(x);
            }
        }

        let n = a.len();
        let mut f = vec![vec![0; 1 << 10]; n + 1];
        f[0][0] = 1;

        for i in 1..=n {
            for j in 0..(1 << 10) {
                f[i][j] = f[i - 1][j];
                let t = st[a[i - 1] as usize];
                if (j & t) == t {
                    f[i][j] = (f[i][j] + f[i - 1][j ^ t]) % 1_000_000_007;
                }
            }
        }

        let mut ret = 0;
        for i in 0..(1 << 10) {
            ret = (ret + f[n][i]) % 1_000_000_007;
        }

        (ret - 1 + 1_000_000_007) % 1_000_000_007

    }
}
