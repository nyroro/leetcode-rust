
// Define a struct named Solution



// Implement the struct Solution

impl Solution {
    // Define a public function named most_frequent_even which takes a vector of i32 and returns an i32

    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        // Create a mutable HashMap to store the frequency of even elements

        let mut freq_map = std::collections::HashMap::new();
        // Initialize the maximum frequency and the most frequent even element

        let (mut max_freq, mut most_frequent_even) = (0, -1);

        // Iterate through the input vector to count the frequency of even elements

        for &num in &nums {
            if num % 2 == 0 {
                let count = freq_map.entry(num).or_insert(0);
                *count += 1;
                // Update the most frequent even element if a new maximum frequency is found

                if *count > max_freq || (*count == max_freq && num < most_frequent_even) {
                    max_freq = *count;
                    most_frequent_even = num;
                }
            }
        }

        // Return the most frequent even element or -1 if no even element is found

        if max_freq > 0 {
            most_frequent_even

        } else {
            -1

        }
    }
}
