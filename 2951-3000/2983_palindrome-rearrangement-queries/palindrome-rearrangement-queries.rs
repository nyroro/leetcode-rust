


impl Solution {
    pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut pr: Vec<Vec<i32>> = vec![vec![0; 26]; n + 1];
        let mut psd: Vec<i32> = vec![0; 1];

        for (i, j) in (0..n).zip((0..n).rev()) {
            psd.push(psd.last().unwrap() + if s[i] != s[j] { 1 } else { 0 });
        }

        for i in 1..=n {
            for j in 0..26 {
                pr[i][j] = pr[i - 1][j];
            }
            pr[i][s[i - 1] as usize - 97] += 1;
        }

        let mut cnt = 0;
        for i in 0..26 {
            if pr[n][i] % 2 != 0 {
                cnt += 1;
            }
        }

        let mut v: Vec<bool> = Vec::new();
        for q in queries {
            let a1 = q[0] as usize + 1;
            let b1 = q[1] as usize + 1;
            let c1 = q[2] as usize + 1;
            let d1 = q[3] as usize + 1;
            let a2 = n - a1 + 1;
            let b2 = n - b1 + 1;
            let c2 = n - c1 + 1;
            let d2 = n - d1 + 1;

            if cnt > 0 {
                v.push(false);
                continue;
            }

            let mut chk = true;
            if a1 <= d2 {
                if d2 >= a1 && c2 <= b1 {
                    if psd[a1 - 1] != 0 || (psd[n / 2] - psd[b1]) != 0 {
                        chk = false;
                    }
                    for i in 0..26 {
                        if pr[b1][i] - pr[a1 - 1][i] != pr[a2][i] - pr[b2 - 1][i] {
                            chk = false;
                        }
                    }
                } else if d2 > b1 {
                    if psd[a1 - 1] != 0 || (psd[d2 - 1] - psd[b1]) != 0 || (psd[n / 2] - psd[c2]) != 0 {
                        chk = false;
                    }
                    for i in 0..26 {
                        if pr[b1][i] - pr[a1 - 1][i] != pr[a2][i] - pr[b2 - 1][i] {
                            chk = false;
                        }
                        if pr[c2][i] - pr[d2 - 1][i] != pr[d1][i] - pr[c1 - 1][i] {
                            chk = false;
                        }
                    }
                } else if b1 >= d2 && c2 >= b1 {
                    if psd[a1 - 1] != 0 || (psd[n / 2] - psd[c2]) != 0 {
                        chk = false;
                    }
                    let mut cnt1: Vec<i32> = vec![0; 26];
                    let mut cnt2: Vec<i32> = vec![0; 26];
                    for i in 0..26 {
                        cnt1[i] = pr[b1][i] - pr[a1 - 1][i];
                        cnt2[i] = pr[d1][i] - pr[c1 - 1][i];
                    }
                    for i in 0..26 {
                        cnt1[i] -= (pr[a2][i] - pr[d1][i]);
                        cnt2[i] -= (pr[c2][i] - pr[b1][i]);
                    }
                    for i in 0..26 {
                        if cnt1[i] < 0 || cnt2[i] < 0 || cnt1[i] != cnt2[i] {
                            chk = false;
                        }
                    }
                }
            } else {
                if a1 > c2 {
                    if psd[d2 - 1] != 0 || (psd[a1 - 1] - psd[c2]) != 0 || (psd[n / 2] - psd[b1]) != 0 {
                        chk = false;
                    }
                    for i in 0..26 {
                        if pr[b1][i] - pr[a1 - 1][i] != pr[a2][i] - pr[b2 - 1][i] {
                            chk = false;
                        }
                        if pr[c2][i] - pr[d2 - 1][i] != pr[d1][i] - pr[c1 - 1][i] {
                            chk = false;
                        }
                    }
                } else if a1 <= c2 && b1 >= c2 {
                    if psd[d2 - 1] != 0 || (psd[n / 2] - psd[b1]) != 0 {
                        chk = false;
                    }
                    let mut cnt1: Vec<i32> = vec![0; 26];
                    let mut cnt2: Vec<i32> = vec![0; 26];
                    for i in 0..26 {
                        cnt1[i] = pr[b1][i] - pr[a1 - 1][i];
                        cnt2[i] = pr[d1][i] - pr[c1 - 1][i];
                    }
                    for i in 0..26 {
                        cnt1[i] -= (pr[c1 - 1][i] - pr[b2 - 1][i]);
                        cnt2[i] -= (pr[a1 - 1][i] - pr[d2 - 1][i]);
                    }
                    for i in 0..26 {
                        if cnt1[i] < 0 || cnt2[i] < 0 || cnt1[i] != cnt2[i] {
                            chk = false;
                        }
                    }
                } else if b1 < c2 {
                    if psd[d2 - 1] != 0 || (psd[n / 2] - psd[c2]) != 0 {
                        chk = false;
                    }
                    for i in 0..26 {
                        if pr[c2][i] - pr[d2 - 1][i] != pr[d1][i] - pr[c1 - 1][i] {
                            chk = false;
                        }
                    }
                }
            }
            v.push(chk);
        }
        v

    }
}
