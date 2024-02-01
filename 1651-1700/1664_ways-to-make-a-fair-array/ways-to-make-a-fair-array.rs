
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum_even = vec![0; n + 1];
        let mut prefix_sum_odd = vec![0; n + 1];
        let mut count = 0;

        for i in 0..n {
            prefix_sum_even[i + 1] = prefix_sum_even[i];
            prefix_sum_odd[i + 1] = prefix_sum_odd[i];
            if i % 2 == 0 {
                prefix_sum_even[i + 1] += nums[i];
            } else {
                prefix_sum_odd[i + 1] += nums[i];
            }
        }

        for i in 0..n {
            let even_sum = prefix_sum_even[i] + prefix_sum_odd[n] - prefix_sum_odd[i + 1];
            let odd_sum = prefix_sum_odd[i] + prefix_sum_even[n] - prefix_sum_even[i + 1];
            if even_sum == odd_sum {
                count += 1;
            }
        }

        count

    }
}
