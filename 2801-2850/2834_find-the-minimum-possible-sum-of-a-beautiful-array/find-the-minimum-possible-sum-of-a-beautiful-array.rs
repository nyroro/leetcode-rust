
const MOD: i64 = 1_000_000_007;



impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let mid: i32 = target >> 1; // take the mid


        if n <= mid {
            return ((n as i64) * ((n + 1) as i64) / 2 % MOD) as i32; // early return

        }

        // take first half

        let mut ans: i64 = (mid as i64) * ((mid + 1) as i64) / 2;
        ans %= MOD;
        // pick rem numbers optimally

        let rem: i32 = n - mid;
        // As per explained expression

        ans += (target as i64) * (rem as i64) + ((rem as i64) * ((rem - 1) as i64) / 2);
        ans %= MOD;
        ans as i32

    }
}
