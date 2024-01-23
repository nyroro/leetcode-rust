


impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = candies_count.len();
        let mut prefix_sum: Vec<i64> = vec![0; n];
        prefix_sum[0] = candies_count[0] as i64;
        
        // 计算前缀和

        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + candies_count[i] as i64;
        }
        
        let mut result = Vec::new();
        
        for query in queries {
            let favorite_type = query[0] as usize;
            let favorite_day = query[1] as i64;
            let daily_cap = query[2] as i64;
            
            let min_candies = if favorite_type == 0 { 1 } else { prefix_sum[favorite_type - 1] + 1 };
            let max_candies = prefix_sum[favorite_type];
            let min_day = favorite_day + 1;
            let max_day = (favorite_day + 1) * daily_cap;
            
            if min_candies <= max_day && max_candies >= min_day {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        
        result

    }
}
