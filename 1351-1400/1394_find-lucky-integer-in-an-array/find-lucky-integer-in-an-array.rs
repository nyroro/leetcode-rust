
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = vec![0; 501];
        
        for &num in &arr {
            freq[num as usize] += 1;
        }
        
        let mut max_lucky = -1;
        
        for (num, &count) in freq.iter().enumerate().skip(1) {
            if num as i32 == count && num as i32 > max_lucky {
                max_lucky = num as i32;
            }
        }
        
        max_lucky

    }
}
