
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        // Initialize the count of operations for strings starting with '0' and '1' respectively

        let mut count_0 = 0;
        let mut count_1 = 0;

        // Convert the string to a character array for easier manipulation

        let chars: Vec<char> = s.chars().collect();

        // Calculate the number of operations needed for strings starting with '0' and '1'
        for (i, &ch) in chars.iter().enumerate() {
            // Calculate the number of operations for strings starting with '0'
            if i % 2 == 0 {
                if ch == '1' {
                    count_0 += 1;
                }
            } else {
                if ch == '0' {
                    count_0 += 1;
                }
            }

            // Calculate the number of operations for strings starting with '1'
            if i % 2 == 0 {
                if ch == '0' {
                    count_1 += 1;
                }
            } else {
                if ch == '1' {
                    count_1 += 1;
                }
            }
        }

        // Return the minimum number of operations needed

        std::cmp::min(count_0, count_1)
    }
}
