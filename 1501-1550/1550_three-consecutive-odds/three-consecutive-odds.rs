
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for i in 0..arr.len() - 2 {
            if arr[i] % 2 != 0 && arr[i + 1] % 2 != 0 && arr[i + 2] % 2 != 0 {
                return true;
            }
        }
        false

    }
}
