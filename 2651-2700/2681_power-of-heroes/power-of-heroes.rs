



impl Solution {
    pub fn sum_of_power(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut t: i64 = 0;
        let mod_val: i64 = 1_000_000_007;

        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        for num in sorted_nums {
            ans = (ans + (t + num as i64) % mod_val * num as i64 % mod_val* num as i64 % mod_val) % mod_val;
            t = (2 * t % mod_val + num as i64) % mod_val;
        }

        ans

    }
}
