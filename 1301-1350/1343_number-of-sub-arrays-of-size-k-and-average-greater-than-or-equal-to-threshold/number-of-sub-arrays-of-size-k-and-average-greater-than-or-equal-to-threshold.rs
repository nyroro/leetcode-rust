
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut sum = 0;
        let mut count = 0;

        while right < arr.len() {
            sum += arr[right];

            if right - left + 1 == k as usize {
                if sum >= threshold * k {
                    count += 1;
                }
                sum -= arr[left];
                left += 1;
            }

            right += 1;
        }

        count

    }
}
