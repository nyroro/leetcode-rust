


impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut cur = 0;
        let mut ans = vec![0; n];
        let mut seen = vec![0; n + 1];

        for i in 0..n {
            if seen[a[i] as usize] == 1 {
                cur += 1;
            }
            seen[a[i] as usize] += 1;

            if seen[b[i] as usize] == 1 {
                cur += 1;
            }
            seen[b[i] as usize] += 1;

            ans[i] = cur;
        }

        ans

    }
}
