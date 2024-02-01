


impl Solution {
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;

        fn gao(arr: &[i32]) -> usize {
            let mut dp = Vec::new();
            for &t in arr {
                if dp.is_empty() {
                    dp.push(t);
                } else if t >= *dp.last().unwrap() {
                    dp.push(t);
                } else {
                    let mut j = dp.binary_search(&t).unwrap_or_else(|x| x);
                    while j+1 
                    dp[j] = t;
                }
            }
            arr.len() - dp.len()
        }

        for i in 0..k {
            ret += gao(&arr[i as usize..]);
        }

        ret as i32

    }
}
