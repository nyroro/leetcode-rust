
// Step 1: Define the Transaction struct

struct Transaction {
    name: String,
    time: i32,
    amount: i32,
    city: String,
}

// Step 2: Implement the parse_transaction function

fn parse_transaction(transaction: &str) -> Transaction {
    let parts: Vec<&str> = transaction.split(',').collect();
    Transaction {
        name: parts[0].to_string(),
        time: parts[1].parse().unwrap(),
        amount: parts[2].parse().unwrap(),
        city: parts[3].to_string(),
    }
}

// Step 3: Implement the is_invalid function

fn is_invalid(transaction: &Transaction, all_transactions: &Vec<Transaction>) -> bool {
    if transaction.amount > 1000 {
        return true;
    }
    for other_transaction in all_transactions {
        if other_transaction.name == transaction.name

            && other_transaction.city != transaction.city

            && (other_transaction.time - transaction.time).abs() <= 60

        {
            return true;
        }
    }
    false

}

// Step 4: Implement the invalid_transactions function

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut all_transactions: Vec<Transaction> = Vec::new();
        for transaction_str in &transactions {
            let transaction = parse_transaction(transaction_str);
            all_transactions.push(transaction);
        }
        let mut invalid_transactions: Vec<String> = Vec::new();
        for transaction in &all_transactions {
            if is_invalid(transaction, &all_transactions) {
                invalid_transactions.push(format!(
                    "{},{},{},{}",
                    transaction.name, transaction.time, transaction.amount, transaction.city

                ));
            }
        }
        invalid_transactions

    }
}
