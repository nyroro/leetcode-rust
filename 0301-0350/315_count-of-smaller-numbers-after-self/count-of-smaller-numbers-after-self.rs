
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut indexed_nums: Vec<(usize, i32)> = nums.iter().cloned().enumerate().collect();
        let mut indices = vec![0; nums.len()];

        Self::merge_sort_count_smaller(&mut indexed_nums, &mut indices, &mut result, 0, nums.len());

        result

    }

    fn merge_sort_count_smaller(
        nums: &mut Vec<(usize, i32)>,
        indices: &mut Vec<usize>,
        result: &mut Vec<i32>,
        start: usize,
        end: usize,
    ) {
        if end - start <= 1 {
            return;
        }

        let mid = start + (end - start) / 2;
        Self::merge_sort_count_smaller(nums, indices, result, start, mid);
        Self::merge_sort_count_smaller(nums, indices, result, mid, end);

        let mut merged = Vec::with_capacity(end - start);
        let mut left = start;
        let mut right = mid;
        let mut right_count = 0;

        while left < mid {
            while right < end && nums[left].1 > nums[right].1 {
                right_count += 1;
                merged.push(nums[right]);
                right += 1;
            }
            result[nums[left].0] += right_count;
            merged.push(nums[left]);
            left += 1;
        }

        while right < end {
            merged.push(nums[right]);
            right += 1;
        }

        nums[start..end].clone_from_slice(&merged);
    }
}
