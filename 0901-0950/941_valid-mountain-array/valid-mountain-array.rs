
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }
        
        let mut i = 0;
        
        // 找到上升序列

        while i < n - 1 && arr[i] < arr[i + 1] {
            i += 1;
        }
        
        // 检查是否存在下降序列

        if i == 0 || i == n - 1 {
            return false;
        }
        
        while i < n - 1 && arr[i] > arr[i + 1] {
            i += 1;
        }
        
        return i == n - 1;
    }
}
