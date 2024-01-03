
impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let mut stones = stones;
        stones.sort(); // 对石头位置进行排序


        let n = stones.len() as i32;
        let max_moves = std::cmp::max(stones[n as usize - 1] - stones[1], stones[n as usize - 2] - stones[0]) - n + 2; // 计算最大移动次数


        let mut min_moves = n; // 初始化最小移动次数为石头的数量

        let mut i = 0;
        for j in 0..n as usize {
            while stones[j] - stones[i as usize] >= n {
                i += 1;
            }
            if j - i as usize + 1 == n as usize - 1 && stones[j] - stones[i as usize] == n as i32 - 2 {
                min_moves = std::cmp::min(min_moves, 2);
            } else {
                min_moves = std::cmp::min(min_moves, n - (j - i as usize + 1) as i32);
            }
        }

        vec![min_moves, max_moves] // 返回最小和最大移动次数

    }
}
