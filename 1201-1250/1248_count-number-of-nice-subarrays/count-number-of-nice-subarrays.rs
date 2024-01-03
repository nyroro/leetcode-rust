
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; nums.len() + 1];
        count[0] = 1;
        let mut ans = 0;
        let mut t = 0;
        for v in nums {
            t += v & 1;
            if t >= k {
                ans += count[(t - k) as usize];
            }
            count[t as usize] += 1;
        }
        ans

    }
}
