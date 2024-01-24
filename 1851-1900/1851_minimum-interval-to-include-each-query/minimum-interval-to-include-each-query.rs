
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; queries.len()];
        let mut ops: Vec<(i32, usize, usize, i32)> = Vec::new();
        let mut in_op = 0;
        let mut out_op = 2;
        let mut query_op = 1;

        for (i, interval) in intervals.iter().enumerate() {
            ops.push((interval[0], in_op, i, interval[1] - interval[0]));
            ops.push((interval[1], out_op, i, 0));
        }

        for (i, q) in queries.iter().enumerate() {
            ops.push((*q, query_op, i, 0));
        }

        ops.sort();

        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        let mut out: std::collections::HashSet<usize> = std::collections::HashSet::new();

        for op in ops {
            if op.1 == in_op {
                heap.push(Reverse((op.3, op.2)));
            } else if op.1 == out_op {
                out.insert(op.2);
            } else {
                while let Some(Reverse((_, idx))) = heap.peek() {
                    if out.contains(&idx) {
                        heap.pop();
                    } else {
                        break;
                    }
                }
                if let Some(Reverse((val, _))) = heap.peek() {
                    ret[op.2] = val + 1;
                }
            }
        }

        ret

    }
}
