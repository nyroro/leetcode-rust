
impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut result: i64 = 0;
        let mut count: i64 = 0;
        let modulo: i64 = 1_000_000_007;
        let mut inventory = inventory;
        inventory.sort_unstable_by(|a, b| b.cmp(a));

        let mut i = 0;
        while count < orders as i64 {
            let curr = inventory[i] as i64;
            let next = if i < inventory.len() - 1 {
                inventory[i + 1] as i64

            } else {
                0

            };
            let diff = curr - next;
            let rounds = (i as i32 + 1) as i64;

            if count + rounds * diff <= orders as i64 {
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
            i += 1;
        }
        result as i32

    }
}
