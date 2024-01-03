
use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    deque: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        FrontMiddleBackQueue {
            deque: VecDeque::new(),
        }
    }
    
    fn push_front(&mut self, val: i32) {
        self.deque.push_front(val);
    }
    
    fn push_middle(&mut self, val: i32) {
        let len = self.deque.len();
        let mid = len / 2;
        self.deque.insert(mid, val);
    }
    
    fn push_back(&mut self, val: i32) {
        self.deque.push_back(val);
    }
    
    fn pop_front(&mut self) -> i32 {
        self.deque.pop_front().unwrap_or(-1)
    }
    
    fn pop_middle(&mut self) -> i32 {
        let len = self.deque.len();
        let mid = len / 2;
        if len % 2 == 0 {
            self.deque.remove(mid - 1).unwrap_or(-1)
        } else {
            self.deque.remove(mid).unwrap_or(-1)
        }
    }
    
    fn pop_back(&mut self) -> i32 {
        self.deque.pop_back().unwrap_or(-1)
    }
}
