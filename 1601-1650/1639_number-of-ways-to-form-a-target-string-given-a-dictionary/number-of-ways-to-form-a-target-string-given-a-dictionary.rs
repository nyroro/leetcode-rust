
// 导入所需的库

use std::collections::HashMap;

// 实现 Solution 结构体



impl Solution {
    // 实现 num_ways 函数

    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        // 定义常量 MOD 用于取模

        const MOD: i64 = 1_000_000_007;
        
        // 获取 words 和 target 的长度

        let (m, n) = (words[0].len(), target.len());
        
        // 初始化 dp 二维数组

        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        // 初始化 count HashMap

        let mut count: HashMap<(usize, char), i64> = HashMap::new();
        
        // 计算每个字符在每个位置出现的次数

        for word in words {
            for (i, c) in word.chars().enumerate() {
                *count.entry((i, c)).or_insert(0) += 1;
            }
        }
        
        // 初始化 dp 数组

        for i in 0..=m {
            dp[i][0] = 1;
        }
        
        // 计算 dp 数组

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1] * count[&(i - 1, target.chars().nth(j - 1).unwrap())] % MOD) % MOD;
            }
        }
        
        // 返回计算结果

        dp[m][n] as i32

    }
}
