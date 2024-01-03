
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort(); // 对数组进行排序

        let n = sorted_arr.len();
        let diff = sorted_arr[1] - sorted_arr[0]; // 计算相邻元素之间的差值

        for i in 1..n {
            if sorted_arr[i] - sorted_arr[i - 1] != diff {
                return false; // 如果有任何两个相邻元素的差值不等于diff，则返回false

            }
        }
        true // 如果所有相邻元素的差值都相等，则返回true

    }
}
