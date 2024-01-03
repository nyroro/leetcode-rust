
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut count = 0;
        let n = arr.len();
        
        for i in 0..n {
            for j in i+1..n {
                for k in j..n {
                    let a = Self::xor_range(&arr, i, j);
                    let b = Self::xor_range(&arr, j, k+1);
                    
                    if a == b {
                        count += 1;
                    }
                }
            }
        }
        
        count

    }
    
    fn xor_range(arr: &Vec<i32>, start: usize, end: usize) -> i32 {
        let mut result = 0;
        
        for i in start..end {
            result ^= arr[i];
        }
        
        result

    }
}
