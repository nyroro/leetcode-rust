
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Engineer {
    efficiency: i64,
    speed: i64,
}

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut engineers: Vec<Engineer> = efficiency.iter().zip(speed.iter())
            .map(|(&eff, &spd)| Engineer { efficiency: eff as i64, speed: spd as i64 })
            .collect();
        engineers.sort_by_key(|e| std::cmp::Reverse(e.efficiency));

        let mut min_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut total_speed: i64 = 0;
        let mut max_performance: i64 = 0;
        let modulo: i64 = 1_000_000_007;

        for engineer in engineers {
            if min_heap.len() == k {
                total_speed -= min_heap.pop().unwrap().0;
            }
            total_speed += engineer.speed;
            min_heap.push(Reverse(engineer.speed));

            let performance = total_speed * engineer.efficiency;
            max_performance = max_performance.max(performance);
        }

        (max_performance % modulo) as i32

    }
}
