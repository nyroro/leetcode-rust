
use std::collections::HashMap;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut count: i64 = 0;
        let modulo: i64 = 1_000_000_007;
        let mut freq: HashMap<i32, i64> = HashMap::new();

        for &num in &arr {
            *freq.entry(num).or_insert(0) += 1;
        }

        for i in 0..arr.len() {
            for j in (i + 1)..arr.len() {
                for k in (j + 1)..arr.len() {
                    let sum = arr[i] + arr[j] + arr[k];
                    if sum == target {
                        count += 1;
                    }
                }
            }
        }

        (count % modulo) as i32

    }
}
