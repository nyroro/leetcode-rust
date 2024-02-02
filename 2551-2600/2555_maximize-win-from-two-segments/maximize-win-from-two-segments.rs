


impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let mut intervals: Vec<(i32, i32)> = Vec::new();
        let mut max_before: Vec<(i32, i32)> = Vec::new();
        let mut start_index = 0;
        let mut count = 0;

        for (index, pos) in prize_positions.iter().enumerate() {
            count += 1;
            while pos - k > prize_positions[start_index] {
                count -= 1;
                start_index += 1;
            }
            intervals.push((count, *pos));
            if max_before.is_empty() || max_before.last().unwrap().0 < count {
                max_before.push((count, *pos));
            }
        }

        let mut max_solution = 0;
        while let Some((count, pos)) = intervals.pop() {
            while let Some(max) = max_before.last() {
                if max.1 >= pos - k {
                    max_before.pop();
                } else {
                    break;
                }
            }
            let candidate = count + if let Some(max) = max_before.last() {
                max.0

            } else {
                0

            };
            max_solution = max_solution.max(candidate);
        }

        max_solution

    }
}
