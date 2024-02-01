
use std::collections::HashMap;

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        nums.sort();
        let mut count_map: HashMap<usize, i32> = HashMap::new();

        for &num in nums.iter().rev() {
            let mut t = num;
            let mut i = 0;
            while t > 0 {
                if t & 1 != 0 {
                    *count_map.entry(i).or_insert(0) += 1;
                }
                t >>= 1;
                i += 1;
            }
        }

        let maxn = *count_map.keys().max().unwrap();
        let mut arr = vec![nums[nums.len() - 1]];

        for &t in nums.iter().rev().skip(1) {
            let n = gao(t, maxn);
            if n == maxn {
                arr.push(t);
            }
        }

        let mut maxm = -1;
        for &t in arr.iter() {
            let mut m = 0;
            for (i, &v) in count_map.iter() {
                if v > 0 {
                    if (t & (1 << i)) == 0 {
                        m += 1 << i;
                    } else {
                        if *count_map.get(&(i + k as usize)).unwrap_or(&0) == 0 {
                            m += 1 << (i + k as usize);
                        } else if *count_map.get(&(i + k as usize)).unwrap_or(&0) == 1

                            && (t & (1 << (i + k as usize))) != 0

                        {
                            m += 1 << (i + k as usize);
                        }
                        if v > 1 {
                            m += 1 << i;
                        }
                    }
                }
            }
            maxm = maxm.max(m);
        }

        maxm as i64

    }

    fn gao(mut t: i32, mut maxn: usize) -> usize {
        let mut i = 0;
        while t > 0 {
            if t & 1 != 0 {
                maxn = maxn.max(i);
            }
            t >>= 1;
            i += 1;
        }
        maxn

    }
}
