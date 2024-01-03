
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        if n == 1 {
            return true;
        }
        if n % 2 == 0 {
            return Solution::is_ugly(n / 2);
        }
        if n % 3 == 0 {
            return Solution::is_ugly(n / 3);
        }
        if n % 5 == 0 {
            return Solution::is_ugly(n / 5);
        }
        return false;
    }
}
