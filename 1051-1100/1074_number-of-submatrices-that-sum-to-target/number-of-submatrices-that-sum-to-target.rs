
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut count = std::collections::HashMap::new();
        let mut result = 0;

        for left in 0..n {
            let mut row_sum = vec![0; m];
            for right in left..n {
                for i in 0..m {
                    row_sum[i] += matrix[i][right];
                }
                let mut prefix_sum = 0;
                count.entry(0).and_modify(|x| *x += 1).or_insert(1);
                for sum in &row_sum {
                    prefix_sum += sum;
                    if let Some(&c) = count.get(&(prefix_sum - target)) {
                        result += c;
                    }
                    count.entry(prefix_sum).and_modify(|x| *x += 1).or_insert(1);
                }
                count.clear();
            }
        }

        result

    }
}
