
impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let mut count: i64 = 0;
        let mut arr = arr;
        arr.sort();

        for i in 0..arr.len() {
            let mut left = i + 1;
            let mut right = arr.len() - 1;

            while left < right {
                let sum = arr[i] + arr[left] + arr[right];
                if sum > target {
                    right -= 1;
                } else if sum < target {
                    left += 1;
                } else {
                    if arr[left] != arr[right] {
                        let mut left_count = 1;
                        let mut right_count = 1;
                        while left + 1 < right && arr[left] == arr[left + 1] {
                            left_count += 1;
                            left += 1;
                        }
                        while right - 1 > left && arr[right] == arr[right - 1] {
                            right_count += 1;
                            right -= 1;
                        }
                        count += left_count * right_count;
                        count %= modulo;
                        left += 1;
                        right -= 1;
                    } else {
                        count += ((right - left + 1) * (right - left) / 2) as i64;
                        count %= modulo;
                        break;
                    }
                }
            }
        }

        count as i32

    }
}
