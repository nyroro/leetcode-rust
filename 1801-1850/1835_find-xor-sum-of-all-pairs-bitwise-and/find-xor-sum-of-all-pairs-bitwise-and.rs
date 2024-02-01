
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            let mut count = 0;
            for j in &arr2 {
                count += ((j >> i) & 1);
            }
            if count % 2 == 1 {
                result |= 1 << i;
            }
        }
        let mut xor_sum = 0;
        for num in &arr1 {
            xor_sum ^= num;
        }
        result & xor_sum

    }
}
