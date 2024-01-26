
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let s1: i64 = nums1.iter().map(|&x| x as i64).sum();
        let s2: i64 = nums2.iter().map(|&x| x as i64).sum();
        
        let z1: usize = nums1.iter().filter(|&t| *t == 0).count();
        let z2: usize = nums2.iter().filter(|&t| *t == 0).count();
        
        if z1 == 0 && z2 == 0 {
            if s1 == s2 {
                return s1;
            } else {
                return -1;
            }
        } else if z1 != 0 && z2 != 0 {
            return std::cmp::max(s1 + z1 as i64, s2 + z2 as i64);
        } else if z1 == 0 {
            if s2 + z2 as i64 <= s1 {
                return s1;
            } else {
                return -1;
            }
        } else { // z2 == 0

            if s1 + z1 as i64 <= s2 {
                return s2;
            } else {
                return -1;
            }
        }
    }
}
