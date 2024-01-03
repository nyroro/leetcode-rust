
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|p| p[1]); // 按照气球的结束位置进行排序

        let mut arrows = 1; // 初始化箭的数量为1

        let mut end = points[0][1]; // 初始化当前箭的结束位置为第一个气球的结束位置
        
        for i in 1..points.len() {
            if points[i][0] > end { // 当前气球的开始位置大于前一个气球的结束位置

                arrows += 1; // 需要一支新的箭

                end = points[i][1]; // 更新当前箭的结束位置

            }
        }
        
        arrows // 返回最小箭数

    }
}
