
use std::collections::HashSet;

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let mut states = HashSet::new();
        let n = n as usize;
        let presses = presses as usize;
        let presses = presses.min(8); // There are only 8 possible states after 8 button presses


        for a in 0..2 {
            for b in 0..2 {
                for c in 0..2 {
                    for d in 0..2 {
                        if a + b + c + d <= presses && (a + b + c + d) % 2 == presses % 2 {
                            let mut state = vec![1; n];
                            if a == 1 {
                                for i in 0..n {
                                    state[i] = 1 - state[i];
                                }
                            }
                            if b == 1 {
                                for i in (0..n).step_by(2) {
                                    state[i] = 1 - state[i];
                                }
                            }
                            if c == 1 {
                                for i in (1..n).step_by(2) {
                                    state[i] = 1 - state[i];
                                }
                            }
                            if d == 1 {
                                for i in (0..n).step_by(3) {
                                    state[i] = 1 - state[i];
                                }
                            }
                            states.insert(state);
                        }
                    }
                }
            }
        }

        states.len() as i32

    }
}
