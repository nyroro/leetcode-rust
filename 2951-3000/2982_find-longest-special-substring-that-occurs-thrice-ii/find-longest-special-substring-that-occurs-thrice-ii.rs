
use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut table: HashMap<char, Vec<usize>> = HashMap::new();

        for (i, t) in s.chars().enumerate() {
            table.entry(t).or_insert(Vec::new()).push(i);
        }

        let mut ret = -1;

        fn gao2(arr: &Vec<usize>, x: usize) -> bool {
            let mut now = 0;
            let mut prev = 0;
            let mut ret = 0;
            for &t in arr {
                if t != prev + 1 {
                    now = 1;
                    prev = t;
                } else {
                    now += 1;
                    prev = t;
                }
                if now >= x {
                    now -= 1;
                    ret += 1;
                    if ret >= 3 {
                        return true;
                    }
                }
            }
            false

        }

        fn gao(arr: &Vec<usize>) -> i32 {
            if arr.len() < 3 {
                return -1;
            }
            let mut l = 1;
            let mut r = arr.len();
            let mut ret = 1;
            while l <= r {
                let mid = (l + r) / 2;
                if gao2(arr, mid) {
                    ret = mid as i32;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            ret

        }

        for arr in table.values() {
            ret = ret.max(gao(arr));
        }
        ret

    }
}
