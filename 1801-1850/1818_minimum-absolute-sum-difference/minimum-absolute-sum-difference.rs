
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let n = nums1.len();
        let mut diff_sum = 0;
        let mut max_diff = 0;
        let mut sorted_nums1 = nums1.clone();
        sorted_nums1.sort();

        for i in 0..n {
            let diff = (nums1[i] - nums2[i]).abs();
            diff_sum = (diff_sum + diff) % modulo;
            let j = match sorted_nums1.binary_search(&nums2[i]) {
                Ok(j) => j,
                Err(j) => j,
            };
            if j < n {
                max_diff = max_diff.max(diff - (sorted_nums1[j] - nums2[i]).abs());
            }
            if j > 0 {
                max_diff = max_diff.max(diff - (sorted_nums1[j - 1] - nums2[i]).abs());
            }
        }

        (diff_sum - max_diff + modulo) % modulo

    }
}
