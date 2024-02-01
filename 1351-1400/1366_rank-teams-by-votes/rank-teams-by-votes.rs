
impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        // 创建一个HashMap来存储每个团队的得票情况

        let mut team_votes = std::collections::HashMap::new();
        
        // 遍历每个选民的投票

        for vote in votes {
            // 遍历每个团队的排名

            for (i, team) in vote.chars().enumerate() {
                // 将团队添加到HashMap中，并增加对应的得票数

                let count = team_votes.entry(team).or_insert(vec![0; 26]);
                count[i] += 1;
            }
        }
        
        // 将团队按照得票数和字母顺序进行排序

        let mut teams: Vec<char> = team_votes.keys().cloned().collect();
        teams.sort_by(|a, b| {
            let count_a = team_votes.get(a).unwrap();
            let count_b = team_votes.get(b).unwrap();
            for i in 0..26 {
                if count_a[i] != count_b[i] {
                    return count_b[i].cmp(&count_a[i]);
                }
            }
            a.cmp(b)
        });
        
        // 将排序后的团队转换为字符串并返回

        teams.iter().collect()
    }
}
