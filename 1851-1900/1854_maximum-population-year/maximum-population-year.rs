
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        // 创建一个长度为101的数组，用于记录每一年的人口数量

        let mut population = vec![0; 101];

        // 遍历logs数组，计算每一年的人口数量

        for log in logs {
            let birth = log[0] - 1950;
            let death = log[1] - 1950;
            for i in birth..death {
                population[i as usize] += 1;
            }
        }

        // 找到人口数量最大的年份，并记录下最早的年份

        let mut max_population = 0;
        let mut earliest_year = 0;
        for (i, count) in population.iter().enumerate() {
            if *count > max_population {
                max_population = *count;
                earliest_year = i;
            }
        }

        // 返回最早的年份

        return earliest_year as i32 + 1950;
    }
}
