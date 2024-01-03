
impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut operations = target[0];
        for i in 1..target.len() {
            operations += (target[i] - target[i - 1]).max(0);
        }
        operations

    }
}
