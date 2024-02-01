
impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = nums1.len() - 1;
        let mut j = nums2.len() - 1;
        let mut max_distance = 0;

        while i >= 0 && j >= 0 {
            if nums1[i] <= nums2[j] {
                max_distance = max_distance.max(j as i32 - i as i32);
            } else {
                i -= 1;
            }
            j -= 1;
        }

        max_distance

    }
}
