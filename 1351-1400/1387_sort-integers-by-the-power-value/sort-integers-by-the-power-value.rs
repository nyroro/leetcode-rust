
impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        // 计算整数的幂值

        fn power(x: i32) -> i32 {
            let mut count = 0;
            let mut num = x;
            while num != 1 {
                if num % 2 == 0 {
                    num /= 2;
                } else {
                    num = 3 * num + 1;
                }
                count += 1;
            }
            count

        }
        
        // 生成区间 [lo, hi] 中所有整数的幂值

        let mut powers: Vec<(i32, i32)> = (lo..=hi).map(|x| (x, power(x))).collect();
        
        // 对幂值进行排序，如果幂值相同，则按照整数值进行排序

        powers.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        
        // 返回排序后的第 k 个整数

        powers[k as usize - 1].0

    }
}
