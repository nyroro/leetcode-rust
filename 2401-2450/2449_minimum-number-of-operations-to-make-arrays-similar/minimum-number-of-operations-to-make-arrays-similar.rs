
impl Solution {
    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let mut nums1 = Vec::new();
        let mut target1 = Vec::new();
        let mut nums2 = Vec::new();
        let mut target2 = Vec::new();
        
        let mut nums = nums;
        let mut target = target;
        
        nums.sort();
        target.sort();
        
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                nums1.push(nums[i]);
            } else {
                nums2.push(nums[i]);
            }
            if target[i] % 2 == 0 {
                target1.push(target[i]);
            } else {
                target2.push(target[i]);
            }
        }
        
        let mut inc: i64 = 0;
        for i in 0..nums1.len() {
            if nums1[i] < target1[i] {
                inc += ((target1[i] - nums1[i]) as i64 / 2);
            }
        }
        
        for i in 0..nums2.len() {
            if nums2[i] < target2[i] {
                inc += ((target2[i] - nums2[i]) as i64 / 2);
            }
        }
        
        inc

    }
}
