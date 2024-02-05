


impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut cost = cost;

        for i in (0..(n / 2)).rev() {
            let left = (i * 2 + 1) as usize;
            let right = (i * 2 + 2) as usize;
            ans += (cost[left] - cost[right]).abs();
            cost[i as usize] += cost[left].max(cost[right]);
        }

        ans

    }
}
