
struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank { balance }
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let idx1 = account1 as usize - 1;
        let idx2 = account2 as usize - 1;
        if idx1 >= self.balance.len() || idx2 >= self.balance.len() {
            return false;
        }
        if self.balance[idx1] < money {
            return false;
        }
        self.balance[idx1] -= money;
        self.balance[idx2] += money;
        true

    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let idx = account as usize - 1;
        if idx >= self.balance.len() {
            return false;
        }
        self.balance[idx] += money;
        true

    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let idx = account as usize - 1;
        if idx >= self.balance.len() {
            return false;
        }
        if self.balance[idx] < money {
            return false;
        }
        self.balance[idx] -= money;
        true

    }
}
