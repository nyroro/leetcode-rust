
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut decrypted = vec![0; n];
        
        for i in 0..n {
            if k > 0 {
                for j in 1..=k {
                    decrypted[i] += code[(i + j as usize) % n];
                }
            } else if k < 0 {
                for j in 1..=(-k) {
                    decrypted[i] += code[(i + n - j as usize) % n];
                }
            }
        }
        
        decrypted

    }
}
