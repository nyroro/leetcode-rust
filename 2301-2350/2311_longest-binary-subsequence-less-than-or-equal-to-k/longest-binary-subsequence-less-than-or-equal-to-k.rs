
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut dp = vec![0; (k + 1) as usize];
        let s = s.chars().collect::<Vec<char>>();
        
        for i in (0..s.len()).rev() {
            let mut t = 1;
            let mut now = 0;
            let mut z = 0;
            let mut o = 0;
            
            for j in (0..dp.len()).rev() {
                if s[i] == '0' {
                    z += 1;
                }
                if now > k || t > k {
                    continue;
                } else if s[i] == '1' {
                    now += t;
                    if now <= k {
                        o += 1;
                        t <<= 1;
                    }
                } else {
                    t <<= 1;
                }
                dp[j] = z + o;
            }
        }
        
        *dp.last().unwrap()
    }
}
