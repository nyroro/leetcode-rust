
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut eaten = 0;

        for (i, (&a, &d)) in apples.iter().zip(days.iter()).enumerate() {
            heap.push((Reverse(i + d), a));
            while let Some(&(top, apples)) = heap.peek() {
                if top.0 <= i as i32 || apples == 0 {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some(&(top, apples)) = heap.peek() {
                if apples > 0 {
                    eaten += 1;
                    heap.pop();
                    heap.push((top, apples - 1));
                }
            }
        }

        let mut now = apples.len() as i32;
        while let Some(&(top, apples)) = heap.peek() {
            if top.0 <= now || apples == 0 {
                heap.pop();
            } else {
                if apples > 0 {
                    eaten += 1;
                    heap.pop();
                    heap.push((top, apples - 1));
                }
                now += 1;
            }
        }

        eaten

    }
}
