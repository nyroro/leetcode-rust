
use std::collections::HashSet;



impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len() / 2;
        let set1: HashSet<_> = nums1.into_iter().collect();
        let set2: HashSet<_> = nums2.into_iter().collect();

        let only_a = std::cmp::min(set1.difference(&set2).count() as i32, n as i32);
        let only_b = std::cmp::min(set2.difference(&set1).count() as i32, n as i32);
        let both = std::cmp::min(set1.intersection(&set2).count() as i32, 2 * n as i32 - only_a - only_b);
        let result = only_a + only_b + both;

        result

    }
}
