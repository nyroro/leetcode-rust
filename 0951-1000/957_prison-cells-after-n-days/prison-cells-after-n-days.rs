
impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut seen = std::collections::HashMap::new();
        let mut is_fast_forwarded = false;
        let mut state = cells.clone();

        for i in 0..n {
            let next_state = Self::next_day(&state);
            let key = next_state.clone();
            if !seen.contains_key(&key) {
                seen.insert(key, i);
                state = next_state;
            } else {
                is_fast_forwarded = true;
                let loop_start = seen.get(&key).unwrap().clone();
                let loop_length = i - loop_start;
                let remaining_days = (n - i) % loop_length;
                return Self::prison_after_n_days(state, remaining_days);
            }
        }

        if !is_fast_forwarded {
            state

        } else {
            state

        }
    }

    fn next_day(cells: &Vec<i32>) -> Vec<i32> {
        let mut next = vec![0; cells.len()];
        for i in 1..cells.len() - 1 {
            next[i] = if cells[i - 1] == cells[i + 1] { 1 } else { 0 };
        }
        next

    }
}
