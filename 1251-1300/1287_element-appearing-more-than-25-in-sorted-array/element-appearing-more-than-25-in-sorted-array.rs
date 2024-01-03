
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let threshold = n / 4;
        
        for i in 0..n - threshold {
            if arr[i] == arr[i + threshold] {
                return arr[i];
            }
        }
        
        -1 // 如果没有找到符合条件的整数，返回-1

    }
}
