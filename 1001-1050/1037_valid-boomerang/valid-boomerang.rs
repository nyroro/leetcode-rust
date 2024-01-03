
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        // 获取三个点的坐标

        let p1 = &points[0];
        let p2 = &points[1];
        let p3 = &points[2];
        
        // 判断三个点是否在同一条直线上

        (p2[1] - p1[1]) * (p3[0] - p2[0]) != (p3[1] - p2[1]) * (p2[0] - p1[0])
    }
}
