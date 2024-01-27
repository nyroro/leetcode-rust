
use std::cmp::Ordering;
impl Solution {
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;

        fn gao(arr: &Vec<&i32>) -> usize {
            let mut dp = Vec::new();
            for &t in arr {
                if dp.is_empty() {
                    dp.push(t);
                } else if t >= *dp.last().unwrap() {
                    dp.push(t);
                } else {
                    let j = dp.binary_search_by(|element| match element.cmp(&t){
                        Ordering::Equal => Ordering::Less,
                        ord => ord,
                    }).unwrap_or_else(|x| x);
                    dp[j] = t;
                }
            }
            arr.len() - dp.len()
        }

        for i in 0..k {
            ret += gao(&arr[i as usize..].iter().step_by(k as usize).collect::<Vec<&i32>>());
        }

        ret as i32

    }
}
