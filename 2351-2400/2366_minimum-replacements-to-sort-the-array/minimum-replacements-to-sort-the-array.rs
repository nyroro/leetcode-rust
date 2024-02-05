
impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut last = nums[n - 1] as i64;  // Initialize 'last' with the last element as i64

        let mut ans = 0;  // Initialize the total operations count


        // Traverse the array in reverse order

        for i in (0..n - 1).rev() {
            if nums[i] as i64 > last {  // If the current element needs replacement

                let t = (nums[i] as i64 / last) + (nums[i] as i64 % last != 0) as i64;  // Calculate how many times the element needs to be divided

                last = nums[i] as i64 / t;  // Update 'last' for the next comparison

                ans += t - 1;  // Add (t - 1) to 'ans' for the number of operations

            } else {
                last = nums[i] as i64;  // Update 'last' without replacement

            }
        }
        ans  // Return the total number of operations

    }
}
