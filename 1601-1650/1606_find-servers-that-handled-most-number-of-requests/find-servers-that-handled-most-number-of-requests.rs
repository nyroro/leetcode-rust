
struct SegmentTree {
    nodes: Vec<i32>,
}

impl SegmentTree {
    fn new(size: usize) -> SegmentTree {
        SegmentTree { nodes: vec![0; size] }
    }

    fn update(&mut self, node: usize, start: usize, end: usize, pos: usize, val: i32) {
        if start > pos || end < pos {
            return;
        }
        if start == end {
            self.nodes[node] = val;
            return;
        }
        let mid = (start + end) / 2;
        self.update(node * 2 + 1, start, mid, pos, val);
        self.update(node * 2 + 2, mid + 1, end, pos, val);
        self.nodes[node] = self.nodes[node * 2 + 1].min(self.nodes[node * 2 + 2]);
    }

    fn get(&self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
        if start > right || end < left {
            return i32::MAX;
        }
        if left <= start && end <= right {
            return self.nodes[node];
        }
        let mid = (start + end) / 2;
        self.get(node * 2 + 1, start, mid, left, right).min(self.get(node * 2 + 2, mid + 1, end, left, right))
    }
}

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let mut seg = SegmentTree::new(4 * k as usize + 4);
        let mut contain = vec![0; k as usize];

        for i in 0..arrival.len() {
            let mut low = (i % k as usize) as i32;
            let mut high = k - 1;
            let mut index = -1;

            while low <= high {
                let mid = (low + high) / 2;

                if seg.get(0, 0, k as usize - 1, i % k as usize, mid as usize) <= arrival[i] {
                    index = mid;
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }

            if index == -1 {
                low = 0;
                high = (i % k as usize) as i32 - 1;
                while low <= high {
                    let mid = (low + high) / 2;
                    if seg.get(0, 0, k as usize - 1, 0, mid as usize) <= arrival[i] {
                        index = mid;
                        high = mid - 1;
                    } else {
                        low = mid + 1;
                    }
                }
            }

            if index != -1 {
                seg.update(0, 0, k as usize - 1, index as usize, arrival[i] + load[i]);
                contain[index as usize] += 1;
            }
        }

        let mut ans = Vec::new();
        let mut maxx = -1;

        for i in 0..k as usize {
            if contain[i] > maxx {
                ans.clear();
                maxx = contain[i];
            }
            if contain[i] == maxx {
                ans.push(i as i32);
            }
        }

        ans

    }
}
