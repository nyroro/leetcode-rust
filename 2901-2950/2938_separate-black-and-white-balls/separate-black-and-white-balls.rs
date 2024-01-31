
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut ans = 0;                    // Variable to store the result

        let mut black = Vec::new();         // Vector to store indices of '1's in the string


        let n = s.len();                    // Length of the input string 's'

        // Iterate through the string 's' to find indices of '1's

        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                black.push(i);              // Store index if the character is '1'
            }
        }

        black.reverse();                    // Reverse the 'black' vector


        // Calculate the minimum steps required

        for (i, &idx) in black.iter().enumerate() {
            // For each '1', calculate steps needed to move it to the end

            ans += (n as i64 - i as i64 - 1 - idx as i64);
        }

        ans  // Return the total minimum steps

    }
}
