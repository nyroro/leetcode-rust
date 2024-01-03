
// Define the Solution struct



impl Solution {
    // Define the array_sign function

    pub fn array_sign(nums: Vec<i32>) -> i32 {
        // Initialize the product variable to store the product of all elements in the array

        let mut product = 1;

        // Iterate through the elements of the array

        for num in &nums {
            // Multiply each element to the product

            product *= num.signum();
        }

        // Return the sign of the product

        product.signum()
    }
}
