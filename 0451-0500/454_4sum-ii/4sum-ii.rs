
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut result = 0;
        
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                let sum = nums1[i] + nums2[j];
                *count_map.entry(sum).or_insert(0) += 1;
            }
        }
        
        for i in 0..nums3.len() {
            for j in 0..nums4.len() {
                let complement = -(nums3[i] + nums4[j]);
                if let Some(count) = count_map.get(&complement) {
                    result += count;
                }
            }
        }
        
        result

    }
}
