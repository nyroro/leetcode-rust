
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        // Convert n to a string for easier digit manipulation

        let n_str = n.to_string();
        let digits = n_str.len();
        let n = n as usize;
        
        // Calculate the count of non-repeated numbers

        let mut count = 0;
        for i in 1..digits {
            count += 9 * Self::permutation(9, i - 1);
        }
        
        // Calculate the count of numbers with repeated digits

        let mut used = [false; 10];
        for (i, digit) in n_str.chars().enumerate() {
            let digit = digit.to_digit(10).unwrap() as usize;
            for j in (if i == 0 { 1 } else { 0 })..digit {
                if !used[j] {
                    count += Self::permutation(10 - (i + 1), digits - (i + 1));
                }
            }
            if used[digit] {
                break;
            }
            used[digit] = true;
            if i == digits - 1 {
                count += 1;
            }
        }
        
        (n - count) as i32

    }
    
    // Calculate the permutation

    fn permutation(n: usize, k: usize) -> usize {
        let mut result = 1;
        for i in 0..k {
            result *= n - i;
        }
        result

    }
}
