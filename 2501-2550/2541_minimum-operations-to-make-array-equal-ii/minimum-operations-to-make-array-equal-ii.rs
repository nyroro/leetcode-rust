
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut s: i64 = 0;
        let mut result: i64 = 0;

        for (a, b) in nums1.iter().zip(nums2.iter()) {
            let diff: i64 = *a as i64 - *b as i64;
            if k == 0 {
                if diff != 0 {
                    return -1;
                }
            } else if diff.abs() % k as i64 != 0 {
                return -1;
            } else {
                s += diff;
                if diff > 0 {
                    result += diff / k as i64;
                }
            }
        }

        if s != 0 {
            return -1;
        } else {
            return result;
        }
    }
}
