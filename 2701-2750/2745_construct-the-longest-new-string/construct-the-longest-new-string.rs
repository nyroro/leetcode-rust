
impl Solution {
    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        let min_xy = std::cmp::min(x, y);
        let mut ans = if x == y {
            2 * min_xy

        } else {
            2 * min_xy + 1

        };
        ans += z;
        ans * 2

    }
}
