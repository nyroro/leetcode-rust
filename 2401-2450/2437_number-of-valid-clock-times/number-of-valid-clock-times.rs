
impl Solution {
    pub fn count_time(time: String) -> i32 {
        // Parse the input time string

        let mut digits: Vec<char> = time.chars().collect();
        let mut count = 0;
        
        // Loop through all possible combinations

        for h1 in 0..10 {
            for h2 in 0..10 {
                if h1 * 10 + h2 < 24 {
                    for m1 in 0..10 {
                        for m2 in 0..10 {
                            if m1 * 10 + m2 < 60 {
                                // Replace the '?' with the current digits

                                digits[0] = if digits[0] == '?' { (h1 + 48) as char } else { digits[0] };
                                digits[1] = if digits[1] == '?' { (h2 + 48) as char } else { digits[1] };
                                digits[3] = if digits[3] == '?' { (m1 + 48) as char } else { digits[3] };
                                digits[4] = if digits[4] == '?' { (m2 + 48) as char } else { digits[4] };
                                
                                // Check if the time is valid

                                let time_str: String = digits.iter().collect();
                                if time_str <= "23:59" {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        count

    }
}
