
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct Building {
    left: i32,
    right: i32,
    height: i32,
}

impl Ord for Building {
    fn cmp(&self, other: &Self) -> Ordering {
        other.height.cmp(&self.height)
    }
}

impl PartialOrd for Building {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lines: Vec<(i32, String, i32)> = Vec::new();
        for building in buildings {
            lines.push((building[0], "in".to_string(), building[2]));
            lines.push((building[1], "out".to_string(), building[2]));
        }
        
        lines.sort();
        let mut inq: BinaryHeap<i32> = BinaryHeap::new();
        let mut outq: BinaryHeap<i32> = BinaryHeap::new();
        let mut pre_top = 0;
        let mut ret: Vec<Vec<i32>> = Vec::new();
        
        for line in lines {
            if line.1 == "in" {
                inq.push(line.2);
            } else if line.1 == "out" {
                outq.push(line.2);
                while let (Some(inq_top), Some(outq_top)) = (inq.peek(), outq.peek()) {
                    if inq_top == outq_top {
                        inq.pop();
                        outq.pop();
                    } else {
                        break;
                    }
                }
            }
            let now_top = *inq.peek().unwrap_or(&0);
            if pre_top != now_top {
                if let Some(last) = ret.last_mut() {
                    if line.0 == last[0] {
                        last[1] = last[1].max(now_top);
                    } else {
                        ret.push(vec![line.0, now_top]);
                    }
                } else {
                    ret.push(vec![line.0, now_top]);
                }
                pre_top = now_top;
            }
        }
        ret

    }
}
