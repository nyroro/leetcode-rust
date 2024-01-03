
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut product = 1;
        let mut count = 0;

        while right < nums.len() {
            product *= nums[right];
            while left <= right && product >= k {
                product /= nums[left];
                left += 1;
            }
            count += right - left + 1;
            right += 1;
        }

        count as i32

    }
}
