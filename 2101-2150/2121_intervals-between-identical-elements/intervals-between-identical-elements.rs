
use std::collections::HashMap;

impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        let mut table: HashMap<i32, Vec<usize>> = HashMap::new();
        let n = arr.len();
        let mut result: Vec<i64> = vec![0; n];

        for (i, &val) in arr.iter().enumerate() {
            table.entry(val).or_insert(vec![]).push(i);
        }

        for (_, indices) in table.iter() {
            if indices.len() == 1 {
                result[indices[0]] = 0;
            } else {
                let mut s = 0;
                for i in 1..indices.len() {
                    s += (indices[i] - indices[0]) as i64;
                }
                result[indices[0]] = s;

                for i in 1..indices.len() {
                    s -= (indices[i] - indices[i - 1]) as i64 * (indices.len() - i) as i64;
                    s += (indices[i] - indices[i - 1]) as i64 * i as i64;
                    result[indices[i]] = s;
                }
            }
        }

        result

    }
}
