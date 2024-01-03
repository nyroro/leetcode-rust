
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut nums: Vec<i32> = Vec::new(); // Create a vector to store the digits

        for c in s.chars() {
            if c.is_digit(10) { // Check if the character is a digit

                nums.push(c.to_digit(10).unwrap() as i32); // Convert the digit character to an integer and store it in the vector

            }
        }
        nums.sort(); // Sort the vector in ascending order

        nums.dedup(); // Remove duplicates from the vector

        if nums.len() < 2 {
            return -1; // If there are less than 2 unique digits, return -1

        } else {
            return nums[nums.len() - 2]; // Return the second largest digit

        }
    }
}
