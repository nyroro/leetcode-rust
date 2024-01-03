
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;
        let n = arr.len();
        
        for i in 0..n {
            for j in (i+1)..n {
                for k in (j+1)..n {
                    if (arr[i] - arr[j]).abs() <= a && (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                        count += 1;
                    }
                }
            }
        }
        
        count

    }
}
