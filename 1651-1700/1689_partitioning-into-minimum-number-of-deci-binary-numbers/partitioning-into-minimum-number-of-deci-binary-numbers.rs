
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let max_digit = n.chars().max().unwrap();
        max_digit.to_digit(10).unwrap() as i32

    }
}
