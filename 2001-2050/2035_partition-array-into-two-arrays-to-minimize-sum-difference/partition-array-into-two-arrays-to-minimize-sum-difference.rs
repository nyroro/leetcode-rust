
use std::collections::{HashMap, HashSet};



impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let s: i32 = nums.iter().sum();
        let ss: i32 = s / 2;
        let n: usize = nums.len() / 2;

        let mut vis: HashSet<i32> = HashSet::new();
        let mut table: HashMap<usize, Vec<i32>> = HashMap::new();

        let mut ts: i32 = nums.iter().take(n).sum();
        for i in 1..(1 << n) - 1 {
            if vis.contains(&((1 << n) - 1 - i)) {
                continue;
            }
            let mut k: usize = 0;
            let mut x: i32 = 0;
            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    x += nums[j];
                    k += 1;
                }
            }
            table.entry(k).or_insert(Vec::new()).push(x);
            table.entry(n - k).or_insert(Vec::new()).push(ts - x);
            vis.insert(x);
        }

        for (_, v) in table.iter_mut() {
            v.sort();
        }

        let mut ret: i32 = 1000000000;
        let mut gao = |x: i32| {
            ret = ret.min((x * 2 - s).abs());
        };

        gao(nums.iter().skip(n).sum());
        vis.clear();
        for i in 1..(1 << n) - 1 {
            if vis.contains(&((1 << n) - 1 - i)) {
                continue;
            }
            let mut k: usize = 0;
            let mut x: i32 = 0;
            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    x += nums[n + j];
                    k += 1;
                }
            }
            if let Some(arr) = table.get(&(n - k)) {
                let k1 = arr.binary_search(&(ss - x)).unwrap_or_else(|e| e);
                let a = k1.saturating_sub(1);
                let b = (k1 + 2).min(arr.len());
                for j in a..b {
                    gao(x + arr[j]);
                }
            }
            vis.insert(i);
        }
        ret

    }
}
