
struct Node {
    l: i32,
    r: i32,
    cnt: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(l: i32, r: i32) -> Node {
        Node {
            l,
            r,
            cnt: 0,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, l: i32, r: i32) -> i32 {
        if self.cnt == self.r - self.l + 1 {
            return self.cnt;
        }
        if l <= self.l && r >= self.r {
            self.cnt = r - l + 1;
            self.left = None;
            self.right = None;
            return self.cnt;
        }
        let mid = (self.l + self.r) / 2;
        if l <= mid {
            let tr = std::cmp::min(mid, r);
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new(self.l, mid)));
            }
            self.left.as_mut().unwrap().add(l, tr);
        }
        if r > mid {
            let tl = std::cmp::max(mid + 1, l);
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(mid + 1, self.r)));
            }
            self.right.as_mut().unwrap().add(tl, r);
        }
        self.cnt = 0;
        if self.left.is_some() {
            self.cnt += self.left.as_ref().unwrap().cnt;
        }
        if self.right.is_some() {
            self.cnt += self.right.as_ref().unwrap().cnt;
        }
        self.cnt

    }
}

struct CountIntervals {
    top: Node,
}

impl CountIntervals {
    fn new() -> CountIntervals {
        CountIntervals {
            top: Node::new(0, 1000000001),
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        self.top.add(left, right);
    }

    fn count(&self) -> i32 {
        self.top.cnt

    }
}
