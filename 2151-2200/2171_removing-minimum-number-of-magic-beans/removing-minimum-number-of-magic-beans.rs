
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans;
        beans.sort(); // 对魔法袋中的魔法豆进行排序

        let mut min_removal = i64::MAX; // 初始化最小移除数量为最大值

        let sum: i64 = beans.iter().map(|&x| x as i64).sum(); // 计算魔法袋中魔法豆的总和

        let n = beans.len() as i64; // 魔法袋的数量


        for i in 0..n {
            let removal = sum - (n - i) * beans[i as usize] as i64; // 计算移除当前位置的魔法豆后的移除数量

            min_removal = min_removal.min(removal); // 更新最小移除数量

        }

        min_removal

    }
}
