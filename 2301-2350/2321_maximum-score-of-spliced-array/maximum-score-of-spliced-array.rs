
impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let original_sum1: i32 = nums1.iter().sum();
        let original_sum2: i32 = nums2.iter().sum();
        
        let diff: Vec<i32> = nums1.iter().zip(nums2.iter()).map(|(a, b)| a - b).collect();
        
        let mut min_negative_sum: i32 = i32::MAX;
        let mut max_positive_sum: i32 = i32::MIN;
        let mut cur_negative_sum: i32 = 0;
        let mut cur_positive_sum: i32 = 0;
        
        for val in diff {
            cur_negative_sum += val;
            if cur_negative_sum > 0 {
                cur_negative_sum = 0;
            }
            
            cur_positive_sum += val;
            if cur_positive_sum < 0 {
                cur_positive_sum = 0;
            }
            
            min_negative_sum = min_negative_sum.min(cur_negative_sum);
            max_positive_sum = max_positive_sum.max(cur_positive_sum);
        }
        
        let result = vec![
            original_sum1 - min_negative_sum,
            original_sum2 + max_positive_sum,
            original_sum1,
            original_sum2,
        ];
        
        *result.iter().max().unwrap()
    }
}
