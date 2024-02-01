
struct ATM {
    banknotes_count: Vec<i32>,
}

impl ATM {
    fn new() -> Self {
        ATM { banknotes_count: vec![0, 0, 0, 0, 0] }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for i in 0..5 {
            self.banknotes_count[i] += banknotes_count[i];
        }
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut ret = vec![0, 0, 0, 0, 0];
        let values = vec![20, 50, 100, 200, 500];
        let mut remaining_amount = amount;

        for i in (0..5).rev() {
            ret[i] = std::cmp::min(remaining_amount / values[i], self.banknotes_count[i]);
            remaining_amount -= ret[i] * values[i];
        }

        if remaining_amount == 0 {
            for i in 0..5 {
                self.banknotes_count[i] -= ret[i];
            }
            ret

        } else {
            vec![-1]
        }
    }
}

fn main() {
    let mut atm = ATM::new();
    atm.deposit(vec![0, 0, 1, 2, 1]);
    println!("{:?}", atm.withdraw(600));
    atm.deposit(vec![0, 1, 0, 1, 1]);
    println!("{:?}", atm.withdraw(600));
    println!("{:?}", atm.withdraw(550));
}
