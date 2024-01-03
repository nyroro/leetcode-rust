
impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const MOD: i32 = 1337;
        let mut res = 1;
        let mut a = a % MOD;
        
        for num in b {
            res = (Self::pow_mod(res, 10, MOD) * Self::pow_mod(a, num, MOD)) % MOD;
        }
        
        res

    }
    
    fn pow_mod(mut base: i32, mut exp: i32, modulus: i32) -> i32 {
        let mut res = 1;
        base %= modulus;
        
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % modulus;
            }
            base = (base * base) % modulus;
            exp /= 2;
        }
        
        res

    }
}
