
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        fn merge_and_count(nums: &mut [i32], left: usize, mid: usize, right: usize) -> usize {
            let mut count = 0;
            let mut i = left;
            let mut j = mid + 1;
            
            while i <= mid && j <= right {
                if nums[i] as i64 > 2 * nums[j] as i64 {
                    count += mid - i + 1;
                    j += 1;
                } else {
                    i += 1;
                }
            }
            
            let mut temp = Vec::new();
            let mut i = left;
            let mut j = mid + 1;
            
            while i <= mid && j <= right {
                if nums[i] <= nums[j] {
                    temp.push(nums[i]);
                    i += 1;
                } else {
                    temp.push(nums[j]);
                    j += 1;
                }
            }
            
            while i <= mid {
                temp.push(nums[i]);
                i += 1;
            }
            
            while j <= right {
                temp.push(nums[j]);
                j += 1;
            }
            
            nums[left..=right].copy_from_slice(&temp);
            
            count

        }
        
        fn merge_sort_and_count(nums: &mut [i32], left: usize, right: usize) -> usize {
            if left >= right {
                return 0;
            }
            
            let mid = left + (right - left) / 2;
            let mut count = merge_sort_and_count(nums, left, mid) + merge_sort_and_count(nums, mid + 1, right);
            count += merge_and_count(nums, left, mid, right);
            
            count

        }
        
        let mut nums = nums;
        let n = nums.len();
        
        merge_sort_and_count(&mut nums, 0, n - 1) as i32

    }
}
