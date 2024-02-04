
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        fn merge_sort(nums: &mut [i32], diff: i32) -> i64 {
            if nums.len() <= 1 {
                return 0;
            }
            let mid = nums.len() / 2;
            let (left, right) = nums.split_at_mut(mid);

            let mut count = merge_sort(left, diff) + merge_sort(right, diff);

            let (mut i, mut j) = (0, 0);
            while i < left.len() && j < right.len() {
                if left[i] as i64 <= right[j] as i64 + diff as i64 {
                    count += (right.len() - j) as i64;
                    i += 1;
                } else {
                    j += 1;
                }
            }

            let mut merged = Vec::new();
            let (mut i, mut j) = (0, 0);
            while i < left.len() && j < right.len() {
                if right[j] < left[i] {
                    merged.push(right[j]);
                    j += 1;
                } else {
                    merged.push(left[i]);
                    i += 1;
                }
            }
            merged.extend_from_slice(&left[i..]);
            merged.extend_from_slice(&right[j..]);
            nums.copy_from_slice(&merged);

            count

        }

        let mut diff_arr: Vec<i32> = nums1.iter().zip(nums2.iter()).map(|(a, b)| a - b).collect();
        merge_sort(&mut diff_arr, diff)
    }
}
