
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut table: HashMap<i32, i32> = HashMap::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let inf: i32 = 0x7fffffff;

        let mut gao = |x: i32, dis: i32| {
            if *table.get(&x).unwrap_or(&inf) > dis + 1 {
                table.insert(x, dis);
                if x >= 0 && x <= 1000 {
                    queue.push_back((dis, x));
                }
            }
        };

        queue.push_back((0, start));
        table.insert(start, 0);

        while let Some((dis, now)) = queue.pop_front() {
            if now == goal {
                continue;
            }
            if dis >= *table.get(&goal).unwrap_or(&inf) {
                continue;
            }
            if dis > *table.get(&now).unwrap_or(&inf) {
                continue;
            }

            if now >= 0 && now <= 1000 {
                for t in &nums {
                    let k1 = now + t;
                    let k2 = now - t;
                    let k3 = now ^ t;
                    gao(k1, dis + 1);
                    gao(k2, dis + 1);
                    gao(k3, dis + 1);
                }
            }
        }

        *table.get(&goal).unwrap_or(&-1)
    }
}
