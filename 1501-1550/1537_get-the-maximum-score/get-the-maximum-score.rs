
impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let (mut p1, mut p2, mut sum1, mut sum2, mut result) = (0, 0, 0, 0, 0i64);

        while p1 < nums1.len() || p2 < nums2.len() {
            if p1 < nums1.len() && (p2 == nums2.len() || nums1[p1] < nums2[p2]) {
                sum1 += nums1[p1] as i64;
                p1 += 1;
            } else if p2 < nums2.len() && (p1 == nums1.len() || nums2[p2] < nums1[p1]) {
                sum2 += nums2[p2] as i64;
                p2 += 1;
            } else {
                result = (result + std::cmp::max(sum1, sum2) + nums1[p1] as i64) % modulo;
                sum1 = 0;
                sum2 = 0;
                p1 += 1;
                p2 += 1;
            }
        }

        result = (result + std::cmp::max(sum1, sum2)) % modulo;
        result as i32

    }
}
