
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut indexed_nums: Vec<(i32, usize)> = nums.iter().cloned().enumerate().map(|(i, t)| (t, i)).collect();
        indexed_nums.sort_by_key(|&(t, _)| t);
        let result: Vec<i32> = indexed_nums.iter().rev().take(k as usize).map(|&(t, _)| t).collect();
        let mut indices: Vec<usize> = indexed_nums.iter().rev().take(k as usize).map(|&(_, i)| i).collect();
        indices.sort();
        result.iter().zip(indices).map(|(&t, i)| nums[i]).collect()
    }
}
