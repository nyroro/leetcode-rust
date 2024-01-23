
impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut o = vec![0; s.len()];
        let mut z = vec![0; s.len()];
        let mut o1 = vec![0; s.len()];
        let mut z1 = vec![0; s.len()];
        
        let mut ret = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            if chars[i] == '0' {
                z[i] = 1;
            } else {
                o[i] = 1;
            }
            if i > 0 {
                z[i] += z[i-1];
                o[i] += o[i-1];
                if chars[i] == '0' {
                    z1[i] = o[i-1];
                    ret += o1[i-1];
                } else {
                    o1[i] = z[i-1];
                    ret += z1[i-1];
                }
                z1[i] += z1[i-1];
                o1[i] += o1[i-1];
            }
        }
        ret

    }
}
