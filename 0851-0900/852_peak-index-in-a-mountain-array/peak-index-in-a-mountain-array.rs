
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = arr.len() - 1;
        
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            
            if arr[mid] < arr[mid + 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        
        lo as i32

    }
}
