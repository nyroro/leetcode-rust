
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        // Step 1: Clone the input array

        let mut nums_copy = nums.clone();
        
        // Step 2: Sort the cloned array

        nums_copy.sort();
        
        // Step 3: Calculate the product differences

        let n = nums_copy.len();
        let max_product = nums_copy[n - 1] * nums_copy[n - 2];
        let min_product = nums_copy[0] * nums_copy[1];
        
        // Step 4: Return the maximum product difference

        max_product - min_product

    }
}
