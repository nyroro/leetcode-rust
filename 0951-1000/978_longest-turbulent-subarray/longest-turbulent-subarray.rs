
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut up = vec![1; n];
        let mut down = vec![1; n];
        let mut max_len = 1;

        for i in 1..n {
            if arr[i] > arr[i - 1] {
                up[i] = down[i - 1] + 1;
                down[i] = 1;
            } else if arr[i] < arr[i - 1] {
                down[i] = up[i - 1] + 1;
                up[i] = 1;
            }

            max_len = max_len.max(up[i]).max(down[i]);
        }

        max_len

    }
}
