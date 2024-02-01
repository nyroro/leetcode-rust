
impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let mut left: i64 = -1_000_000_000;
        let mut right: i64 = 1_000_000_000;
        
        while left < right {
            let mid = left + (right - left) / 2;
            let mut negative_count = 0;
            let mut total_count = 0;
            
            for i in 0..nums1.len() {
                if nums1[i] > 0 {
                    let mut low = 0;
                    let mut high = nums2.len() as i32 - 1;
                    while low <= high {
                        let m = low + (high - low) / 2;
                        if (nums1[i] * nums2[m as usize]) <= mid {
                            low = m + 1;
                        } else {
                            high = m - 1;
                        }
                    }
                    total_count += low as i64;
                } else if nums1[i] < 0 {
                    let mut low = 0;
                    let mut high = nums2.len() as i32 - 1;
                    while low <= high {
                        let m = low + (high - low) / 2;
                        if (nums1[i] * nums2[m as usize]) <= mid {
                            high = m - 1;
                        } else {
                            low = m + 1;
                        }
                    }
                    total_count += (nums2.len() as i64 - low as i64);
                    negative_count += (nums2.len() as i64 - low as i64);
                } else if nums2[i] == 0 {
                    if mid >= 0 {
                        total_count += nums2.len() as i64;
                    }
                }
            }
            
            if total_count < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        left

    }
}
