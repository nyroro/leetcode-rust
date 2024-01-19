
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let n = customers.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut total_customers = 0;

        // 计算前缀和

        for (i, c) in customers.chars().enumerate() {
            if c == 'Y' {
                total_customers += 1;
            }
            prefix_sum[i + 1] = total_customers;
        }

        let mut min_penalty = n as i32;
        let mut best_closing_time = 0;

        // 计算每个小时关闭店铺所产生的惩罚值

        for i in 0..=n {
            let penalty = (i as i32 - prefix_sum[i]) + (total_customers - prefix_sum[i]);
            if penalty < min_penalty {
                min_penalty = penalty;
                best_closing_time = i as i32;
            }
        }

        best_closing_time

    }
}
