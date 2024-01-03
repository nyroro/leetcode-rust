
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        // Calculate the median of the array

        let median = n / 2;
        
        // Calculate the sum of absolute differences between each element and the median

        let mut operations = 0;
        for i in 0..n {
            operations += (2 * i + 1 - (2 * median + 1)).abs();
        }
        
        // Return the sum of absolute differences divided by 2

        operations / 2

    }
}
