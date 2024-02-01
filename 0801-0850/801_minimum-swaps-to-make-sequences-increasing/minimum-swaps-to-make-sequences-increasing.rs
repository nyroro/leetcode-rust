
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut n = 0;
        let mut s = 1;
        
        for i in 1..nums1.len() {
            let (mut new_n, mut new_s) = (std::i32::MAX, std::i32::MAX);
            
            if nums1[i] > nums1[i-1] && nums2[i] > nums2[i-1] {
                new_n = n;
                new_s = s;
            }
            
            if nums1[i] > nums2[i-1] && nums2[i] > nums1[i-1] {
                new_n = new_n.min(s);
                new_s = new_s.min(n + 1);
            }
            
            n = new_n;
            s = new_s;
        }
        
        n.min(s)
    }
}
