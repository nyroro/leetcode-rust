


impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let mut s = nums.clone();
        for i in 1..nums.len() {
            s[i] += s[i - 1];
        }
        let mod_val = 1_000_000_007;
        let mut ret = 0;
        let n = s.len();
        let (mut l, mut r) = (0, 0);
        for i in 2..n {
            if s[i - 1] <= 2 * (s[n - 1] - s[i - 1]) {
                while l < i - 2 && s[i - 1] - s[l] > s[n - 1] - s[i - 1] {
                    l += 1;
                }
                r = r.max(l);
                while r < i - 1 && s[r] <= s[i - 1] - s[r] {
                    r += 1;
                }
                ret += r - l;
                ret %= mod_val;
            }
        }
        ret as i32

    }
}
