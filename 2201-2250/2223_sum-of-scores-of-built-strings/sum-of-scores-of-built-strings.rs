


impl Solution {
    pub fn sum_scores(s: String) -> i64 {
        let mut l = 0;
        let mut r = 0;
        let n = s.len();
        let mut z: Vec<usize> = vec![0; n];
        z[0] = n;
        let mut s_chars: Vec<char> = s.chars().collect();

        for i in 1..n {
            if i > r {
                l = i;
                r = i;
                while r < n && s_chars[r - l] == s_chars[r] {
                    r += 1;
                }
                z[i] = r - l;
                r -= 1;
            } else {
                let k = i - l;
                if z[k] < r - i + 1 {
                    z[i] = z[k];
                } else {
                    l = i;
                    while r < n && s_chars[r - l] == s_chars[r] {
                        r += 1;
                    }
                    z[i] = r - l;
                    r -= 1;
                }
            }
        }

        z.iter().map(|&x| x as i64).sum()
    }
}
