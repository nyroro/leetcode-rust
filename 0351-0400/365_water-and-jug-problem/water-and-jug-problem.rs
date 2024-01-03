
impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![(0, 0)];

        while let Some((x, y)) = stack.pop() {
            if x + y == target_capacity {
                return true;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            // 将 x 壶倒满

            stack.push((jug1_capacity, y));
            // 将 y 壶倒满

            stack.push((x, jug2_capacity));
            // 将 x 壶倒空

            stack.push((0, y));
            // 将 y 壶倒空

            stack.push((x, 0));
            // 将 x 壶的水倒入 y 壶，直到 x 壶为空或者 y 壶满

            stack.push((x - std::cmp::min(x, jug2_capacity - y), y + std::cmp::min(x, jug2_capacity - y)));
            // 将 y 壶的水倒入 x 壶，直到 y 壶为空或者 x 壶满

            stack.push((x + std::cmp::min(y, jug1_capacity - x), y - std::cmp::min(y, jug1_capacity - x)));
        }

        false

    }
}
