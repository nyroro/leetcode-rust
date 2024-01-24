
impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut s = vec![nums[0] as i64];
        for i in 1..n {
            s.push(nums[i] as i64 + s[i-1]);
        }
        let mut ret = Vec::new();
        for q in queries {
            let t = nums.binary_search(&q).unwrap_or_else(|x| x);
            if t == 0 {
                ret.push(s[n-1] - q as i64 * n as i64);
            } else if t == n {
                ret.push(q as i64 * n as i64 - s[n-1]);
            } else {
                let l = q as i64 * t as i64 - s[t-1];
                let r = s[n-1] - s[t-1] - q as i64 * (n - t) as i64;
                ret.push(l + r);
            }
        }
        ret

    }
}
