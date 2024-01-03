
// 创建一个名为Solution的结构体



impl Solution {
    // 实现一个名为total_money的公共函数

    pub fn total_money(n: i32) -> i32 {
        // 计算完整的周数和剩余的天数

        let weeks = n / 7;
        let days = n % 7;
        
        // 计算完整的周数存款总额

        let full_weeks_total = 28 * weeks + 7 * (weeks - 1) * weeks / 2;
        
        // 计算剩余天数的存款总额

        let partial_week_total = (weeks + 1..=weeks + days).sum::<i32>();
        
        // 返回总金额

        full_weeks_total + partial_week_total

    }
}
