
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut prefix_sum = 0;
        let mut remainder_count = vec![0; k as usize];
        remainder_count[0] = 1;

        for num in nums {
            prefix_sum = (prefix_sum + num) % k;
            if prefix_sum < 0 {
                prefix_sum += k;
            }
            count += remainder_count[prefix_sum as usize];
            remainder_count[prefix_sum as usize] += 1;
        }

        count

    }
}
