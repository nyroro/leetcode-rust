
impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut good_transactions = Vec::new();
        let mut bad_transactions = Vec::new();

        for txn in &transactions {
            if txn[0] <= txn[1] {
                good_transactions.push(txn);
            } else {
                bad_transactions.push(txn);
            }
        }

        bad_transactions.sort_by_key(|&txn| txn[1]);

        let mut need = 0;
        let mut cur_amount = 0;

        for txn in &bad_transactions {
            cur_amount += i64::from(txn[0]);
            need = need.max(cur_amount);
            cur_amount -= i64::from(txn[1]);
        }

        let max_good_transaction = good_transactions.iter().map(|txn| txn[0]).max();
        if let Some(costliest_good_transaction) = max_good_transaction {
            cur_amount += i64::from(costliest_good_transaction);
            need = need.max(cur_amount);
        }

        need

    }
}
