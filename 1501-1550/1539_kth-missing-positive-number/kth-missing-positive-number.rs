
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut left = 1;
        let mut right = 0;
        let mut count = 0;
        
        while count < k && right < arr.len() {
            if arr[right] == left {
                left += 1;
                right += 1;
            } else {
                count += 1;
                left += 1;
            }
        }
        
        if count < k {
            return arr[arr.len() - 1] + k - count;
        } else {
            return left - 1;
        }
    }
}
