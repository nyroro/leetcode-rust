
impl Solution {
    pub fn max_profit(mut inventory: Vec<i32>, orders: i32) -> i32 {
        let mut result: i64 = 0;
        let mut count: i64 = 0;
        inventory.sort_unstable_by(|a, b| b.cmp(a));
        let modulo = 1_000_000_007;

        for i in 0..inventory.len() {
            let curr = inventory[i];
            if i < inventory.len() - 1 {
                let next = inventory[i + 1];
                let diff = curr - next;
                let rounds = (i as i32 + 1) as i64;
                if count + rounds * diff < orders as i64 {
                    let total = (curr + next + 1) * diff / 2;
                    result = (result + total * rounds) % modulo;
                    count += rounds * diff;
                } else {
                    let rounds = (orders as i64 - count) / (i as i64 + 1);
                    let remainder = (orders as i64 - count) % (i as i64 + 1);
                    let total = (curr + curr - rounds + 1) * rounds / 2;
                    result = (result + total * (i as i64 + 1) + (curr - rounds) * remainder) % modulo;
                    break;
                }
            } else {
                let rounds = orders as i64 - count;
                let total = (curr + curr - rounds + 1) * rounds / 2;
                result = (result + total) % modulo;
                break;
            }
        }
        result as i32

    }
}
