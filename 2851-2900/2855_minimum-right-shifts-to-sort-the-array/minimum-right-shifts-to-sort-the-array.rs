
impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        // Helper function to check if the array is sorted

        fn is_sorted(arr: &Vec<i32>) -> bool {
            for i in 0..arr.len() - 1 {
                if arr[i] > arr[i + 1] {
                    return false;
                }
            }
            true

        }

        let n = nums.len();
        let mut nums = nums;
        for i in 0..n {
            if is_sorted(&nums) {
                return i as i32;
            }
            let last = nums[n - 1];
            for j in (1..n).rev() {
                nums[j] = nums[j - 1];
            }
            nums[0] = last;
        }
        -1

    }
}
