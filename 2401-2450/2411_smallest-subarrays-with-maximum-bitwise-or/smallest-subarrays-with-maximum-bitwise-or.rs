
use std::collections::HashMap;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ret = vec![1; n];
        let mut arr: Vec<HashMap<i32, i32>> = vec![HashMap::new(); n];
        let mut r = n - 1;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        fn gao(cnta: &HashMap<i32, i32>, cntb: &HashMap<i32, i32>) -> bool {
            let mut cnt = cnta.clone();
            for (k, v) in cntb.iter() {
                *cnt.entry(*k).or_insert(0) -= v;
            }
            let sa: Vec<i32> = cnta.iter().filter(|(_, &v)| v > 0).map(|(k, _)| *k).collect();
            let s: Vec<i32> = cnt.iter().filter(|(_, &v)| v > 0).map(|(k, _)| *k).collect();
            sa.len() == s.len()
        }

        for i in (0..n).rev() {
            let mut t = nums[i];
            while t > 0 {
                *arr[i].entry(t & -t).or_insert(0) += 1;
                t -= t & -t;
            }
            for (k, v) in &arr[i] {
                *cnt.entry(*k).or_insert(0) += v;
            }
            while r > i && gao(&cnt, &arr[r]) {
                for (k, v) in &arr[r] {
                    *cnt.entry(*k).or_insert(0) -= v;
                }
                r -= 1;
            }
            ret[i] = (r - i + 1) as i32;
        }
        ret

    }
}
