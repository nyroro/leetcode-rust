
impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        let max_achievable = (num - t).max(num + t);
        max_achievable

    }
}
