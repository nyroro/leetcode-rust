
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let n = arr.len();
        
        for start in 0..n {
            for end in start..n {
                if (end - start + 1) % 2 == 1 {
                    let subarray = &arr[start..=end];
                    result += subarray.iter().sum::<i32>();
                }
            }
        }
        
        result

    }
}
