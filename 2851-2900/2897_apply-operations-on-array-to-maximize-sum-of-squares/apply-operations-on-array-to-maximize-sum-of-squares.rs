
impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MODU: i64 = 1_000_000_007;
        let mut cnt = [0; 31];
        
        for &num in &nums {
            for j in 0..31 {
                let bit = (num >> j) & 1;
                if bit == 1 {
                    cnt[j] += 1;
                }
            }
        }
        
        let mut v = vec![0; k as usize];
        for i in 0..k as usize {
            for j in 0..31 {
                if cnt[j] > 0 {
                    cnt[j] -= 1;
                    v[i] |= 1 << j;
                }
            }
        }
        
        let mut sum: i64 = 0;
        for &num in &v {
            sum = (sum + (num as i64 * num as i64) % MODU) % MODU;
        }
        
        sum as i32

    }
}
