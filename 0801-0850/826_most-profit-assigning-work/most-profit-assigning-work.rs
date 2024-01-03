
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit.into_iter()).collect();
        jobs.sort_unstable(); // 按照难度的升序排序

        let mut max_profit = 0;
        let mut max_profit_map: Vec<i32> = vec![0; 100001]; // 利润的最大值映射表，根据题目给定的约束条件

        let mut max_profit_so_far = 0;
        
        for (d, p) in jobs {
            max_profit_so_far = max_profit_so_far.max(p); // 更新当前难度下的最大利润

            max_profit_map[d as usize] = max_profit_so_far; // 将最大利润保存到映射表中

        }
        
        for w in worker {
            let mut found = false;
            for i in (0..=w as usize).rev() {
                if max_profit_map[i] > 0 {
                    max_profit += max_profit_map[i];
                    found = true;
                    break;
                }
            }
            if !found {
                max_profit += 0;
            }
        }
        
        max_profit

    }
}
