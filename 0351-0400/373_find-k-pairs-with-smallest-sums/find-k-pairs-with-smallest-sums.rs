
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut min_heap = BinaryHeap::new();

        for &num1 in nums1.iter() {
            for &num2 in nums2.iter() {
                min_heap.push(Reverse(num1 + num2));
                if min_heap.len() > k as usize {
                    min_heap.pop();
                }
            }
        }

        while let Some(Reverse(sum)) = min_heap.pop() {
            for &num1 in nums1.iter() {
                for &num2 in nums2.iter() {
                    if num1 + num2 == sum {
                        result.push(vec![num1, num2]);
                    }
                }
            }
        }

        result

    }
}
