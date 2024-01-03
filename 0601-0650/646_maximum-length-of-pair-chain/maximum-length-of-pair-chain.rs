
impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_by_key(|pair| pair[1]); // 按照右边界进行升序排序

        let n = pairs.len();
        let mut dp = vec![1; n]; // dp数组，记录以每个pair结尾的最长链的长度

        for i in 1..n {
            for j in 0..i {
                if pairs[i][0] > pairs[j][1] {
                    dp[i] = dp[i].max(dp[j] + 1); // 更新dp数组

                }
            }
        }
        *dp.iter().max().unwrap() // 返回dp数组中的最大值

    }
}
