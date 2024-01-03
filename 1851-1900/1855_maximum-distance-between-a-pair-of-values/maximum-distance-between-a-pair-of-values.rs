
impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut max_distance = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] {
                max_distance = max_distance.max(j as i32 - i as i32);
                j += 1;
            } else {
                i += 1;
            }
        }

        max_distance

    }
}
