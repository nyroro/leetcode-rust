
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let m = nums1.len();
        let n = nums2.len();
        let total_len = m + n;
        let mid = (total_len + 1) / 2;

        let mut left = 0;
        let mut right = m;

        while left <= right {
            let i = (left + right) / 2;
            let j = mid - i;

            if i < m && nums2[j - 1] > nums1[i] {
                left = i + 1;
            } else if i > 0 && nums1[i - 1] > nums2[j] {
                right = i - 1;
            } else {
                let max_left = if i == 0 {
                    nums2[j - 1]
                } else if j == 0 {
                    nums1[i - 1]
                } else {
                    nums1[i - 1].max(nums2[j - 1])
                };

                if total_len % 2 == 1 {
                    return max_left as f64;
                }

                let min_right = if i == m {
                    nums2[j]
                } else if j == n {
                    nums1[i]
                } else {
                    nums1[i].min(nums2[j])
                };

                return (max_left + min_right) as f64 / 2.0;
            }
        }

        0.0

    }
}
