
impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        // 计算四个点之间的距离

        let d2 = |p1: &[i32], p2: &[i32]| -> i32 {
            (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)
        };

        let mut distances = vec![
            d2(&p1, &p2),
            d2(&p1, &p3),
            d2(&p1, &p4),
            d2(&p2, &p3),
            d2(&p2, &p4),
            d2(&p3, &p4),
        ];

        // 对距离进行排序

        distances.sort_unstable();

        // 判断是否为正方形

        distances[0] > 0 && distances[0] == distances[1] && distances[1] == distances[2]
            && distances[2] == distances[3] && distances[3] == distances[0]
            && distances[4] == distances[5]
    }
}
