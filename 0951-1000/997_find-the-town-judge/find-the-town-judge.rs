
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        // 创建一个数组用于记录每个人被信任的次数

        let mut trust_count = vec![0; n as usize + 1];
        
        // 遍历 trust 数组，统计每个人被信任的次数

        for t in trust {
            trust_count[t[0] as usize] -= 1; // 信任别人的次数减一

            trust_count[t[1] as usize] += 1; // 被信任的次数加一

        }
        
        // 遍历 trust_count 数组，找到被信任次数为 n-1 的人，即法官

        for i in 1..=n as usize {
            if trust_count[i] == n - 1 {
                return i as i32; // 返回法官的标签

            }
        }
        
        -1 // 如果没有找到法官，则返回 -1

    }
}
