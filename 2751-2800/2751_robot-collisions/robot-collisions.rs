
struct Robot {
    position: i32,
    health: i32,
    direction: char,
}

impl Solution {
    pub fn perform_operations(healths: &mut Vec<i32>, st: &mut Vec<usize>, i: usize) {
        while let Some(top) = st.pop() {
            if healths[top] > healths[i] {
                healths[top] -= 1;
                healths[i] = 0;
                st.push(top);
                return;
            } else if healths[top] == healths[i] {
                healths[i] = 0;
                healths[top] = 0;
                return;
            } else {
                healths[i] -= 1;
                healths[top] = 0;
            }
        }
    }

    pub fn survived_robots_healths(positions: Vec<i32>, mut healths: Vec<i32>, directions: String) -> Vec<i32> {
        let n = positions.len();
        let mut robots: Vec<Robot> = (0..n).map(|i| Robot {
            position: positions[i],
            health: healths[i],
            direction: directions.as_bytes()[i] as char, // Convert string to bytes and access by index

        }).collect();

        let mut st: Vec<usize> = Vec::new();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by_key(|&i| robots[i].position);

        for &i in &idx {
            if robots[i].direction == b'L' as char && st.is_empty() {
                continue;
            } else if robots[i].direction == 'R' {
                st.push(i);
            } else {
                Solution::perform_operations(&mut healths, &mut st, i); // Pass the entire healths vector

            }
        }

        let ans: Vec<i32> = healths.into_iter().filter(|&x| x > 0).collect();
        ans

    }
}
