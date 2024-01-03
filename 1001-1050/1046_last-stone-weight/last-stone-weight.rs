
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        stones.sort_by(|a, b| b.cmp(a)); // 按照降序排序


        while stones.len() > 1 {
            let heaviest = stones.remove(0);
            let second_heaviest = stones.remove(0);

            if heaviest != second_heaviest {
                stones.push(heaviest - second_heaviest);
                stones.sort_by(|a, b| b.cmp(a)); // 每次碰撞后重新排序

            }
        }

        if stones.is_empty() {
            0

        } else {
            stones[0]
        }
    }
}
