
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut result: i64 = 0;
        let modulo: i64 = 1000000007;
        
        for i in 1..=n {
            let binary = format!("{:b}", i);
            let bits = binary.len() as i64;
            
            result = (result * (1 << bits) % modulo + i as i64) % modulo;
        }
        
        result as i32

    }
}
