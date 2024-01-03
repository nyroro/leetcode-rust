
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut times = Vec::new(); // 创建一个新的数组来存储所有蚂蚁到达末端的时间点

        for pos in left {
            times.push(pos); // 计算左侧蚂蚁到达末端的时间点，并添加到数组中

        }
        for pos in right {
            times.push(n - pos); // 计算右侧蚂蚁到达末端的时间点（因为它们是反向移动），并添加到数组中

        }
        times.iter().max().unwrap_or(&0).clone() // 找到数组中的最大值，即为最后一只蚂蚁到达末端的时间点

    }
}
