
impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        match num1.checked_add(num2) {
            Some(result) => result,
            None => i32::MAX,
        }
    }
}
