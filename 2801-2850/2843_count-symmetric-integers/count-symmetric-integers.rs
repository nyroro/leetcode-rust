
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        // variable to store the count

        let mut count = 0;

        // Loop until i is less than or equal to high, and greater than or equal to low

        for i in low..=high {
            // If the number is symmetric increase the count

            if Solution::is_symmetric(i) {
                count += 1;
            }
        }
        return count;  // return the count

    }

    // function to check if the number is symmetric

    fn is_symmetric(num: i32) -> bool {
        let s = num.to_string();
        let n = s.len();

        // If the length of the string is odd, not possible

        if n % 2 == 1 {
            return false;
        }

        let mut left_sum = 0;
        let mut right_sum = 0;

        for i in 0..n/2 {
            left_sum += s.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            right_sum += s.chars().nth(n - i - 1).unwrap().to_digit(10).unwrap() as i32;
        }

        // return true if the left sum is equal to the right sum

        return left_sum == right_sum;
    }
}
