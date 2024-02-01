
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        // 创建一个变量来记录比赛的总场次

        let mut total_matches = 0;
        // 当队伍数量大于1时，进行比赛

        while n > 1 {
            // 如果队伍数量为偶数

            if n % 2 == 0 {
                // 计算比赛场次，并将晋级的队伍数量赋值给n

                total_matches += n / 2;
                n /= 2;
            } else {
                // 如果队伍数量为奇数

                // 计算比赛场次，并将晋级的队伍数量赋值给n

                total_matches += (n - 1) / 2;
                n = (n - 1) / 2 + 1;
            }
        }
        // 返回总的比赛场次

        total_matches

    }
}
