
impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut is_covered = vec![false; 51]; // 创建一个长度为 51 的布尔数组，初始值为 false

        for range in ranges {
            for i in range[0]..=range[1] {
                is_covered[i as usize] = true; // 将区间范围内的整数标记为已覆盖

            }
        }
        for i in left..=right {
            if !is_covered[i as usize] {
                return false; // 如果存在未被覆盖的整数，则返回 false

            }
        }
        true // 如果所有整数都被覆盖，则返回 true

    }
}
