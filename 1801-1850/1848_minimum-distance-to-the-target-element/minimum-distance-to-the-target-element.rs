
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        // 初始化最小距离为一个较大的值

        let mut min_distance = i32::MAX;

        // 遍历数组nums，查找与target相等的元素

        for (i, num) in nums.iter().enumerate() {
            if *num == target {
                // 计算当前索引与start的距离

                let distance = (i as i32 - start).abs();
                // 更新最小距离

                min_distance = min_distance.min(distance);
            }
        }

        // 返回最小距离

        min_distance

    }
}
