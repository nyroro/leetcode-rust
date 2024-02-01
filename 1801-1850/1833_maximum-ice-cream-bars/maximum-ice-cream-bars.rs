
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut count = vec![0; 100001];
        let mut total = 0;
        let mut max_ice_cream = 0;
        
        for &price in &costs {
            count[price as usize] += 1;
        }
        
        for i in 1..=100000 {
            if total + i as i32 * count[i] as i32 <= coins {
                total += i as i32 * count[i] as i32;
                max_ice_cream += count[i];
            } else {
                let remaining_coins = coins - total;
                max_ice_cream += (remaining_coins / i as i32) as usize;
                break;
            }
        }
        
        max_ice_cream as i32

    }
}
