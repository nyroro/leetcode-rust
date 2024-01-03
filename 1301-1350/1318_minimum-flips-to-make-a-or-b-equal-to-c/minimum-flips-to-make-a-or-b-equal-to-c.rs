
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut flips = 0;
        let mut a = a;
        let mut b = b;
        let mut c = c;
        
        for i in 0..32 {
            let bit_a = a & 1;
            let bit_b = b & 1;
            let bit_c = c & 1;
            
            if bit_c == 0 {
                if bit_a == 1 {
                    flips += 1;
                }
                if bit_b == 1 {
                    flips += 1;
                }
            } else {
                if bit_a == 0 && bit_b == 0 {
                    flips += 1;
                }
            }
            
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        
        flips

    }
}
