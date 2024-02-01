


impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort(); // Sort the nums array

        let mut operations = 0;
        
        while nums[nums.len() - 1] > 0 {
            let mut min_non_zero = 101; // Initialize with a value greater than the maximum possible value in nums

            for &num in &nums {
                if num > 0 && num < min_non_zero {
                    min_non_zero = num;
                }
            }
            operations += 1;
            for num in &mut nums {
                if *num > 0 {
                    *num -= min_non_zero;
                }
            }
        }
        
        operations

    }
}

fn main() {
    // Test cases

    let nums1 = vec![1, 5, 0, 3, 5];
    let nums2 = vec![0];
    println!("Minimum operations for nums1: {}", Solution::minimum_operations(nums1));
    println!("Minimum operations for nums2: {}", Solution::minimum_operations(nums2));
}
