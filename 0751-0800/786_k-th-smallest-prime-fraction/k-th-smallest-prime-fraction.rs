
impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len();
        let mut left = 0.0;
        let mut right = 1.0;
        let mut result = vec![0, 0];

        while right - left > 1e-9 {
            let mid = (left + right) / 2.0;
            let mut count = 0;
            let mut j = 1;
            let mut best = 0.0;

            for i in 0..n {
                while j < n && arr[i] as f64 > mid * arr[j] as f64 {
                    j += 1;
                }
                if j == n {
                    break;
                }
                count += n - j;
                let value = arr[i] as f64 / arr[j] as f64;
                if value > best {
                    best = value;
                    result[0] = arr[i];
                    result[1] = arr[j];
                }
            }

            if count == k {
                break;
            } else if count < k {
                left = mid;
            } else {
                right = mid;
            }
        }

        result

    }
}
