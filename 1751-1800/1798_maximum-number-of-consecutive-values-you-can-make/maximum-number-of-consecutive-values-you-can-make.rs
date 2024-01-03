
impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        coins.sort(); // 对硬币数组进行排序

        let mut max_value = 0; // 当前能够形成的最大连续整数值


        for coin in coins {
            if coin <= max_value + 1 {
                max_value += coin;
            } else {
                break;
            }
        }

        max_value + 1

    }
}
