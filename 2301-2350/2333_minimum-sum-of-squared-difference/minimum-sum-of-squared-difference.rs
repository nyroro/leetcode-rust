
impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut diff_values: Vec<i64> = vec![0; 100001]; // Initialize a vector to store frequency of differences


        // Calculate absolute differences and store their frequency

        for (v1, v2) in nums1.iter().zip(nums2.iter()) {
            let diff = (*v1 - *v2).abs() as usize;
            diff_values[diff] += 1;
        }

        let mut k = k1 as i64 + k2 as i64; // Convert k1 and k2 to i64

        let mut total: i64 = 0;

        // Minimize the sum of squared differences

        let mut i = 100000;
        while k > 0 && i > 0 {
            while i > 0 && diff_values[i] == 0 {
                i -= 1;
            }
            if i == 0 {
                break; // Exit the loop if no non-zero frequency is found

            }
            let freq = diff_values[i];
            if k >= freq {
                k -= freq;
                diff_values[i] = 0;
                diff_values[i - 1] += freq;
            } else {
                diff_values[i] -= k;
                diff_values[i - 1] += k;
                k = 0;
            }
        }

        // Calculate the total sum of squared differences

        for (val, &count) in diff_values.iter().enumerate() {
            total += (val as i64 * val as i64 * count);
        }

        total

    }
}
