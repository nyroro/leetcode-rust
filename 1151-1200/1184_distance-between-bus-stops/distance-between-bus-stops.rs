
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let (start, destination) = (start as i32, destination as i32);
        let (mut clockwise, mut counterclockwise) = (0, 0);

        // 计算顺时针方向上的距离

        let mut i = start;
        while i != destination {
            clockwise += distance[i as usize];
            i = (i + 1) % n;
        }

        // 计算逆时针方向上的距离

        let mut j = start;
        while j != destination {
            j = (j - 1 + n) % n;
            counterclockwise += distance[j as usize];
        }

        // 返回最短距离

        clockwise.min(counterclockwise)
    }
}
