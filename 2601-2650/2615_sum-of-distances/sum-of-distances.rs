
use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut d: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            d.entry(num).or_insert(Vec::new()).push(i);
        }

        let mut answ: Vec<i64> = vec![0; nums.len()];

        for val in d.values() {
            let mut suffix_sum = val.iter().sum::<usize>() as i64;
            let mut prefix_sum = 0;
            let mut s = val.len();
            let mut p = 0;

            for &i in val {
                prefix_sum += i as i64;
                p += 1;
                suffix_sum -= i as i64;
                s -= 1;
                answ[i] = -(prefix_sum) + p as i64 * i as i64 - s as i64 * i as i64 + suffix_sum;
            }
        }

        answ

    }
}
