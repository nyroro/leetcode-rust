
impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        // 创建一个结构体来表示玩家和训练师的能力值和训练容量

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        struct Ability(i32);

        // 将玩家和训练师的数组转换为能力值结构体的数组

        let mut players: Vec<Ability> = players.into_iter().map(Ability).collect();
        let mut trainers: Vec<Ability> = trainers.into_iter().map(Ability).collect();

        // 对玩家和训练师的数组进行排序

        players.sort();
        trainers.sort();

        // 使用贪心算法进行匹配

        let mut matches = 0;
        let mut trainer_idx = 0;
        for player in &players {
            while trainer_idx < trainers.len() && trainers[trainer_idx] < *player {
                trainer_idx += 1;
            }
            if trainer_idx < trainers.len() {
                matches += 1;
                trainer_idx += 1;
            }
        }

        // 返回匹配数作为结果

        matches

    }
}
