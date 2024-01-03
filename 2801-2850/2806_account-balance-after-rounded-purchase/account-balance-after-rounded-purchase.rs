
impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let rounded_amount = ((purchase_amount + 5) / 10) * 10;
        100 - rounded_amount

    }
}
