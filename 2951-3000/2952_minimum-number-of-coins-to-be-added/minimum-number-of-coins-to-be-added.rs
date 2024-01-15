
impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let mut coins = coins;
        coins.sort();
        let mut max_reachable = 0;
        let mut added_coins = 0;
        let mut i = 0;

        while max_reachable < target {
            if i < coins.len() && coins[i] <= max_reachable + 1 {
                max_reachable += coins[i];
                i += 1;
            } else {
                max_reachable += max_reachable + 1;
                added_coins += 1;
            }
        }

        added_coins

    }
}
