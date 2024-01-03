
impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let mut count = 0;  // 计数器变量，初始值为 0

        for &num in &nums {  // 遍历数组中的每个元素

            let mut has_smaller = false;  // 标记是否存在严格较小的元素

            let mut has_larger = false;  // 标记是否存在严格较大的元素

            for &other_num in &nums {  // 遍历整个数组

                if other_num < num {  // 检查是否存在严格较小的元素

                    has_smaller = true;
                } else if other_num > num {  // 检查是否存在严格较大的元素

                    has_larger = true;
                }
            }
            if has_smaller && has_larger {  // 如果同时存在严格较小和严格较大的元素

                count += 1;  // 计数器加一

            }
        }
        count  // 返回计数器的值作为结果

    }
}
