
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort(); // 对数组进行排序


        let mut min_diff = i32::MAX; // 初始化最小差值为无穷大

        let mut result = Vec::new(); // 创建空数组来保存结果


        for i in 1..sorted_arr.len() {
            let diff = sorted_arr[i] - sorted_arr[i - 1]; // 计算差值


            if diff < min_diff {
                min_diff = diff; // 更新最小差值

                result.clear(); // 清空结果数组

                result.push(vec![sorted_arr[i - 1], sorted_arr[i]]); // 添加新的元素对

            } else if diff == min_diff {
                result.push(vec![sorted_arr[i - 1], sorted_arr[i]]); // 添加符合条件的元素对

            }
        }

        result // 返回结果数组

    }
}
