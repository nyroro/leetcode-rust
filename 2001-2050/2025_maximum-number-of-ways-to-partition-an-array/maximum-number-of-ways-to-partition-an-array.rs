
use std::collections::HashMap;

impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let tot: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut ways = vec![0; n];
        let mut mp = HashMap::new();

        let mut cur = nums[0] as i64;
        *mp.entry(cur).or_insert(0) += 1;

        for i in 1..n {
            let delta = k as i64 - nums[i] as i64;
            let new_tot = tot + delta;

            if new_tot % 2 == 0 {
                ways[i] += *mp.get(&(new_tot / 2)).unwrap_or(&0);
            }

            cur += nums[i] as i64;
            *mp.entry(cur).or_insert(0) += 1;
        }

        mp.clear();
        cur = nums[n - 1] as i64;
        *mp.entry(cur).or_insert(0) += 1;

        for i in (0..n - 1).rev() {
            let delta = k as i64 - nums[i] as i64;
            let new_tot = tot + delta;

            if new_tot % 2 == 0 {
                ways[i] += *mp.get(&(new_tot / 2)).unwrap_or(&0);
            }

            cur += nums[i] as i64;
            *mp.entry(cur).or_insert(0) += 1;
        }

        if tot % 2 == 0 {
            *mp.get_mut(&cur).unwrap() -= 1;
            ways.push(*mp.get(&(tot / 2)).unwrap_or(&0));
        }

        *ways.iter().max().unwrap() as i32

    }
}
