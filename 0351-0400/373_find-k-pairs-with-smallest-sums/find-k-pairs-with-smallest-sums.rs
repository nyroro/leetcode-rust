use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut min_heap = BinaryHeap::new();

        if nums1.is_empty() || nums2.is_empty() {
            return result;
        }

        for i in 0..nums1.len() {
            min_heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }

        while result.len() < k as usize && !min_heap.is_empty() {
            let Reverse((sum, i, j)) = min_heap.pop().unwrap();
            result.push(vec![nums1[i], nums2[j]]);

            if j + 1 < nums2.len() {
                min_heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        result
    }
}
