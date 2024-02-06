
impl Solution {
    pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
        let mut sum = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).sum::<i64>();
        if sum <= target as i64 {
            return 0;
        }
        let (mut zeros, mut diff) = (10, 0);
        while sum > target as i64 {
            diff = zeros - n % zeros;
            zeros *= 10;
            sum = (n + diff).to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).sum::<i64>();
        }
        diff

    }
}
