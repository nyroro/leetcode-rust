
// Define a struct named Solution



// Implement the minimum_time function within the Solution struct

impl Solution {
    // Function to calculate the minimum time

    fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let n = nums1.len();
        let mut ind: Vec<usize> = (0..n).collect();
        let mut s = 0;
        let mut d = 0;

        for i in 0..n {
            s += nums1[i];
            d += nums2[i];
        }

        if s <= x {
            return 0;
        }

        // Custom comparator for sorting ind based on nums2 values

        ind.sort_by_key(|&i| nums2[i]);

        let mut dp: Vec<i32> = vec![0; n + 1];
        let mut r = n + 1;

        for i in 1..=n {
            for j in (1..=r - 1).rev() {
                dp[j] = dp[j].max(dp[j - 1] + (nums2[ind[i - 1]] * j as i32) + nums1[ind[i - 1]]);
                if s + j as i32 * d - dp[j] <= x {
                    r = j;
                }
            }
        }

        if r <= n {
            return r as i32;
        } else {
            return -1;
        }
    }
}
