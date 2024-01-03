
impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut count = 0;
        
        // 遍历 nums1 数组

        for i in 0..nums1.len() {
            // 计算 nums1[i] 的平方

            let square = nums1[i] as i64 * nums1[i] as i64;
            
            // 遍历 nums2 数组，查找满足条件的三元组

            for j in 0..nums2.len() {
                for k in j + 1..nums2.len() {
                    if square == nums2[j] as i64 * nums2[k] as i64 {
                        count += 1;
                    }
                }
            }
        }
        
        // 遍历 nums2 数组

        for i in 0..nums2.len() {
            // 计算 nums2[i] 的平方

            let square = nums2[i] as i64 * nums2[i] as i64;
            
            // 遍历 nums1 数组，查找满足条件的三元组

            for j in 0..nums1.len() {
                for k in j + 1..nums1.len() {
                    if square == nums1[j] as i64 * nums1[k] as i64 {
                        count += 1;
                    }
                }
            }
        }
        
        count

    }
}
