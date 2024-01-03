
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; (n + 1) as usize];
        for i in 1..=n as usize {
            ans[i] = ans[i >> 1] + (i as i32 & 1);
        }
        ans

    }
}
