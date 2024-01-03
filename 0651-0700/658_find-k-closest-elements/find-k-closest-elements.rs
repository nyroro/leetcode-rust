
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = arr.len() - 1;
        
        while right - left + 1 > k as usize {
            let left_diff = (arr[left] - x).abs();
            let right_diff = (arr[right] - x).abs();
            
            if left_diff > right_diff {
                left += 1;
            } else {
                right -= 1;
            }
        }
        
        arr[left..=right].to_vec()
    }
}
