
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        fn merge_count(nums: &mut Vec<i32>, start: usize, mid: usize, end: usize) -> i32 {
            let mut count = 0;
            let mut temp = vec![0; end - start + 1];
            let (mut i, mut j, mut k) = (start, mid + 1, 0);
            
            while i <= mid && j <= end {
                if nums[i] > nums[j] {
                    temp[k] = nums[j];
                    count += (mid - i + 1) as i32;
                    j += 1;
                } else {
                    temp[k] = nums[i];
                    i += 1;
                }
                k += 1;
            }
            
            while i <= mid {
                temp[k] = nums[i];
                i += 1;
                k += 1;
            }
            
            while j <= end {
                temp[k] = nums[j];
                j += 1;
                k += 1;
            }
            
            for i in 0..temp.len() {
                nums[start + i] = temp[i];
            }
            
            count

        }
        
        fn merge_sort(nums: &mut Vec<i32>, start: usize, end: usize) -> i32 {
            if start >= end {
                return 0;
            }
            let mid = start + (end - start) / 2;
            let mut count = merge_sort(nums, start, mid) + merge_sort(nums, mid + 1, end);
            count += merge_count(nums, start, mid, end);
            count

        }
        
        let mut nums = nums.clone();
        let global_inversions = merge_sort(&mut nums, 0, nums.len() - 1);
        let local_inversions = nums.iter().zip(nums.iter().skip(1)).filter(|(&a, &b)| a > b).count() as i32;
        
        global_inversions == local_inversions

    }
}
