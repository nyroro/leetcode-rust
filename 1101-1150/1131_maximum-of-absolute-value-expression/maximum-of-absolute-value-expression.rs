
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let n = arr1.len();
        let mut max_val = 0;

        for i in 0..n {
            for j in 0..n {
                let val = (arr1[i] - arr1[j]).abs() + (arr2[i] - arr2[j]).abs() + (i as i32 - j as i32).abs();
                max_val = max_val.max(val);
            }
        }

        max_val

    }
}
