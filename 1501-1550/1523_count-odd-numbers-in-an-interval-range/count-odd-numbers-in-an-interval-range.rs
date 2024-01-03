
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let count = (high - low) / 2;
        if low % 2 != 0 || high % 2 != 0 {
            count + 1

        } else {
            count

        }
    }
}
