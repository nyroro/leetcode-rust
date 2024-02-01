
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut global_inversions = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] > nums[j] {
                    global_inversions += 1;
                }
            }
        }
        
        let mut local_inversions = 0;
        for i in 0..(nums.len() - 1) {
            if nums[i] > nums[i + 1] {
                local_inversions += 1;
            }
        }
        
        global_inversions == local_inversions

    }
}
