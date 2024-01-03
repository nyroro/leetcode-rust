
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max_chunks = 0;
        let mut max_value = 0;
        
        for (i, &num) in arr.iter().enumerate() {
            max_value = max_value.max(num);
            
            if max_value == i as i32 {
                max_chunks += 1;
            }
        }
        
        max_chunks

    }
}
