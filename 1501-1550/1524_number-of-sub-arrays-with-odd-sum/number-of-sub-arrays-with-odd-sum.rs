
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut odd_sum_count = 0;
        let mut prefix_sum = 0;
        let mut prefix_sum_count = vec![0; 2];
        prefix_sum_count[0] = 1;

        for num in arr {
            prefix_sum = (prefix_sum + num) % 2;
            prefix_sum_count[prefix_sum as usize] += 1;
            odd_sum_count = (odd_sum_count + prefix_sum_count[(1 - prefix_sum) as usize]) % MOD;
        }

        odd_sum_count

    }
}
