
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = 0;
        let mut right = n - 1;

        // 找到需要移除的最短子数组的左边界

        while left < n - 1 && arr[left] <= arr[left + 1] {
            left += 1;
        }

        // 如果整个数组已经是非递减的，直接返回 0

        if left >= right {
            return 0;
        }

        // 找到需要移除的最短子数组的右边界

        while right > 0 && arr[right] >= arr[right - 1] {
            right -= 1;
        }

        let mut res = std::cmp::min(n - left - 1, right);

        // 对于每个 i，计算当前子数组的长度并更新最短子数组的长度

        for i in 0..=left {
            if arr[i] <= arr[right] {
                res = std::cmp::min(res, right - i - 1);
            }
        }

        for i in (right..n).rev() {
            if arr[i] >= arr[left] {
                res = std::cmp::min(res, i - left - 1);
            }
        }

        res as i32

    }
}
