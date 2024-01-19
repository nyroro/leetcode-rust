
impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort();
        if amount[0] + amount[1] <= amount[2] {
            return amount[2];
        }
        (amount[0] + amount[1] + amount[2] + 1) / 2

    }
}
