
// Define the Solution struct



impl Solution {
    // Define the array_sign function

    pub fn array_sign(nums: Vec<i32>) -> i32 {
        // Initialize the product variable to store the product of all elements in the array

        let mut product = 1;

        // Iterate through the elements of the array

        for num in nums {
            // Multiply each element to the product

            product *= num;
        }

        // Check the sign of the product and return the corresponding value

        if product > 0 {
            return 1;
        } else if product < 0 {
            return -1;
        } else {
            return 0;
        }
    }
}
