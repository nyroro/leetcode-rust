
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        
        if 6 * nums1.len() < nums2.len() || 6 * nums2.len() < nums1.len() {
            return -1; // impossible

        }
        
        if nums1.iter().sum::<i32>() < nums2.iter().sum::<i32>() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        
        let mut s1 = nums1.iter().sum::<i32>();
        let mut s2 = nums2.iter().sum::<i32>();
        
        nums1.sort();
        nums2.sort();
        
        let (mut ans, mut j) = (0, 0);
        let mut i = nums1.len() as i32 - 1;
        
        while s1 > s2 {
            if j >= nums2.len() || (i >= 0 && nums1[i as usize] - 1 > 6 - nums2[j]) {
                s1 += 1 - nums1[i as usize];
                i -= 1;
            } else {
                s2 += 6 - nums2[j];
                j += 1;
            }
            ans += 1;
        }
        
        ans

    }
}
