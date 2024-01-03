
impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut max_gap = 0;
        let mut prev = bottom - 1; // 初始化前一个特殊楼层的位置为bottom之前的一个位置

        let mut special = special; // 将特殊楼层的数组进行排序

        special.sort();

        for floor in special {
            let gap = floor - prev - 1; // 计算当前特殊楼层和前一个特殊楼层之间的楼层数

            max_gap = max_gap.max(gap); // 更新最大连续楼层数

            prev = floor; // 更新前一个特殊楼层的位置

        }

        let last_gap = top - prev; // 计算最后一个特殊楼层到top之间的楼层数

        max_gap.max(last_gap) // 返回最大连续楼层数

    }
}
