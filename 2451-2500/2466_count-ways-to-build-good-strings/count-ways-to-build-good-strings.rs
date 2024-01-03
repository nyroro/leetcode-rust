
// 请将以下代码添加到您的解决方案中



impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut f: Vec<i32> = vec![-1; (high + 1) as usize];
        let lo = low as usize;
        let hi = high as usize;
        let z = zero as usize;
        let o = one as usize;
        Solution::dfs(0, &mut f, lo, hi, z, o) as i32

    }

    fn dfs(i: usize, f: &mut Vec<i32>, lo: usize, hi: usize, z: usize, o: usize) -> i32 {
        if i > hi {
            return 0;
        }
        if f[i] != -1 {
            return f[i];
        }
        let mut ans: i64 = 0;
        if i >= lo && i <= hi {
            ans += 1;
        }
        ans += Solution::dfs(i + z, f, lo, hi, z, o) as i64 + Solution::dfs(i + o, f, lo, hi, z, o) as i64;
        ans %= 1000000007;
        f[i] = ans as i32;
        ans as i32

    }
}
