
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut max_length = 0;
        let n = arr.len();
        let mut i = 1;

        while i < n - 1 {
            if arr[i] > arr[i - 1] && arr[i] > arr[i + 1] {
                let mut left = i - 1;
                let mut right = i + 1;

                while left > 0 && arr[left] > arr[left - 1] {
                    left -= 1;
                }

                while right < n - 1 && arr[right] > arr[right + 1] {
                    right += 1;
                }

                let length = right - left + 1;
                max_length = max_length.max(length);
            }

            i += 1;
        }

        max_length

    }
}
