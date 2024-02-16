
use std::collections::BinaryHeap;
use std::cmp::Reverse;



impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut vp: Vec<(i32, i32)> = nums2.iter().cloned().zip(nums1.iter().cloned()).collect();
        vp.sort_by_key(|&(a, _)| Reverse(a));

        let mut ans: i64 = 0;
        let mut sum: i64 = 0;
        let mut pq = BinaryHeap::new();

        for (j, (v2, v1)) in vp.iter().enumerate() {
            sum += *v1 as i64;
            pq.push(Reverse(*v1 as i64));
            if pq.len() < k {
                continue;
            }
            if pq.len() == k {
                ans = ans.max((sum * (*v2 as i64)) as i64);
                sum -= pq.pop().unwrap().0;
            }
        }

        ans

    }
}
