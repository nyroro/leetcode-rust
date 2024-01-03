
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut max_sum = i32::MIN;

        for left in 0..cols {
            let mut row_sum = vec![0; rows];
            for right in left..cols {
                for i in 0..rows {
                    row_sum[i] += matrix[i][right];
                }
                max_sum = std::cmp::max(max_sum, Self::max_sum_subarray(&row_sum, k));
                if max_sum == k {
                    return k;
                }
            }
        }

        max_sum

    }

    fn max_sum_subarray(nums: &[i32], k: i32) -> i32 {
        let mut max_sum = i32::MIN;
        let mut prefix_sum = 0;
        let mut set = std::collections::BTreeSet::new();
        set.insert(0);

        for &num in nums {
            prefix_sum += num;
            if let Some(&prev_sum) = set.range(prefix_sum - k..).next() {
                max_sum = std::cmp::max(max_sum, prefix_sum - prev_sum);
            }
            set.insert(prefix_sum);
        }

        max_sum

    }
}
