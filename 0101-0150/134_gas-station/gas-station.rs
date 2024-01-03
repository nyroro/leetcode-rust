
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gas = 0;
        let mut current_gas = 0;
        let mut start = 0;

        for i in 0..gas.len() {
            current_gas += gas[i] - cost[i];
            total_gas += gas[i] - cost[i];

            if current_gas < 0 {
                start = i + 1;
                current_gas = 0;
            }
        }

        if total_gas < 0 {
            -1

        } else {
            start as i32

        }
    }
}
