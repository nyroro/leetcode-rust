
impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let total = machines.iter().sum::<i32>();
        let n = machines.len() as i32;
        if total % n != 0 {
            return -1;
        }
        let avg = total / n;
        let mut moves = 0;
        let mut balance = 0;
        for num in machines {
            balance += num - avg;
            moves = moves.max(num - avg).max(balance.abs());
        }
        moves

    }
}
