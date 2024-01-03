
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        // Create an empty hash set to store elements in nums1 that are not in nums2

        let mut diff_set = std::collections::HashSet::new();
        // Create another empty hash set to store elements in nums2 that are not in nums1

        let mut diff_set2 = std::collections::HashSet::new();
        
        // Iterate through nums1 and nums2 to find the differences

        for &num in &nums1 {
            if !nums2.contains(&num) {
                diff_set.insert(num);
            }
        }
        
        for &num in &nums2 {
            if !nums1.contains(&num) {
                diff_set2.insert(num);
            }
        }
        
        // Convert the hash sets to vectors and return the result

        vec![diff_set.into_iter().collect(), diff_set2.into_iter().collect()]
    }
}
