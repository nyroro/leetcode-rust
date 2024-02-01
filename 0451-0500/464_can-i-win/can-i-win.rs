
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        // Create a vector to store the availability of each number

        let mut used = vec![false; max_choosable_integer as usize + 1];
        // Check if the desired total is less than or equal to 0

        if desired_total <= 0 {
            return true;
        }
        // Check if the sum of all numbers is less than the desired total

        if (max_choosable_integer * (max_choosable_integer + 1)) / 2 < desired_total {
            return false;
        }
        // Create a memoization table

        let mut memo = std::collections::HashMap::new();
        // Call the recursive helper function to determine if the first player can force a win

        Self::can_win(&mut used, desired_total, &mut memo)
    }
    
    fn can_win(used: &mut Vec<bool>, desired_total: i32, memo: &mut std::collections::HashMap<String, bool>) -> bool {
        // Check if the desired total has already been reached

        if desired_total <= 0 {
            return false;
        }
        // Calculate the unique state of the current player's choices

        let state = Self::get_state(used);
        // Check if the current state has already been memoized

        if let Some(&result) = memo.get(&state) {
            return result;
        }
        // Iterate through all possible choices

        for i in 1..used.len() {
            // Check if the current number is available

            if !used[i] {
                // Mark the current number as used

                used[i] = true;
                // Check if the opponent cannot force a win from the next state

                if !Self::can_win(used, desired_total - i as i32, memo) {
                    // Memoize the current state and return true

                    memo.insert(state.clone(), true);
                    used[i] = false;
                    return true;
                }
                // Undo the current choice

                used[i] = false;
            }
        }
        // Memoize the current state and return false

        memo.insert(state.clone(), false);
        false

    }
    
    fn get_state(used: &Vec<bool>) -> String {
        let mut state = String::new();
        for &is_used in used.iter().skip(1) {
            if is_used {
                state.push('1');
            } else {
                state.push('0');
            }
        }
        state

    }
}
