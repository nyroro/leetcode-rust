


impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len();
        let mut arr = arr;
        arr.sort();
        let median = arr[(n - 1) / 2];
        
        arr.sort_by(|&a, &b| {
            let diff_a = (a - median).abs();
            let diff_b = (b - median).abs();
            if diff_a == diff_b {
                b.cmp(&a)
            } else {
                diff_b.cmp(&diff_a)
            }
        });
        
        arr.truncate(k as usize);
        arr

    }
}
