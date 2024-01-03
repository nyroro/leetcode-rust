
impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut total_matches = 0;
        while n > 1 {
            if n % 2 == 0 {
                total_matches += n / 2;
                n /= 2;
            } else {
                total_matches += (n - 1) / 2;
                n = (n - 1) / 2 + 1;
            }
        }
        total_matches

    }
}
