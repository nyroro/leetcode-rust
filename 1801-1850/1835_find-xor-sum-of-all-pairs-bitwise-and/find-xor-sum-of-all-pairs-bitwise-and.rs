
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in &arr1 {
            result ^= i;
        }
        for j in &arr2 {
            result ^= j;
        }
        result

    }
}
