
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let (mut z, mut o) = (0, 0);
        let mut now = 0;
        let mut t = 1;
        
        for &c in s.as_bytes().iter().rev() {
            if c == b'0' {
                z += 1;
            }
            if now > k || t > k {
                continue;
            } else if c == b'1' {
                now += t;
                if now <= k {
                    o += 1;
                    t <<= 1;
                }
            } else {
                t <<= 1;
            }
        }
        
        z + o

    }
}
