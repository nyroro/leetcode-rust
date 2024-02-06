
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        
        // Step 1: Create a closure to check if it's possible to satisfy the conditions

        let is_possible = || -> bool {
            let mut cnt1 = HashMap::new();
            let mut cnt2 = HashMap::new();
            
            for &v in nums1.iter() {
                *cnt1.entry(v).or_insert(0) += 1;
            }
            for &v in nums2.iter() {
                *cnt2.entry(v).or_insert(0) += 1;
            }
            
            for (v, count) in &cnt1 {
                if let Some(cnt) = cnt2.get(v) {
                    if count > &(n - cnt) {
                        return false;
                    }
                }
            }
            
            true

        };
        
        // Step 2: Count the occurrences of each value in nums1 and nums2

        if !is_possible() {
            return -1;
        }
        
        // Step 3: Create a HashSet to store the indices where the values in nums1 and nums2 are equal

        let mut val2bad_inds_cnt = HashMap::new();
        let mut bad_inds = HashSet::new();
        
        for i in 0..n {
            if nums1[i] == nums2[i] {
                *val2bad_inds_cnt.entry(nums1[i]).or_insert(0) += 1;
                bad_inds.insert(i);
            }
        }
        
        let bad_inds_cnt = bad_inds.len();
        
        if bad_inds_cnt == 0 {
            return 0;
        }
        
        // Step 4: Calculate the minimum total cost based on the number of bad indices and the dominant bad value

        let mut ans = bad_inds.iter().sum::<usize>() as i64;
        let mut dominant_bad_value = 0;
        let mut max_count = 0;
        
        for (&v, &count) in &val2bad_inds_cnt {
            if count > max_count {
                max_count = count;
                dominant_bad_value = v;
            }
        }
        
        let pairs = std::cmp::min(bad_inds_cnt / 2, bad_inds_cnt - val2bad_inds_cnt[&dominant_bad_value]);
        
        if pairs != bad_inds_cnt - val2bad_inds_cnt[&dominant_bad_value] {
            return ans;
        }
        
        let mut swaps_left = bad_inds_cnt - pairs * 2;
        
        // Step 5: Calculate the minimum total cost

        let can_swap = |i: &usize| -> bool {
            !bad_inds.contains(i) && nums1[*i] != dominant_bad_value && nums2[*i] != dominant_bad_value

        };
        
        let mut i = 0;
        while swaps_left > 0 {
            while i < n && !can_swap(&i) {
                i += 1;
            }
            ans += i as i64;
            swaps_left -= 1;
            i += 1;
        }
        
        // Step 6: Return the minimum total cost or -1 if it's not possible to satisfy the conditions

        ans

    }
}
