
use std::collections::VecDeque;

struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.queue1.is_empty() {
            self.queue2.push_back(x);
        } else {
            self.queue1.push_back(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.queue1.is_empty() {
            while self.queue2.len() > 1 {
                if let Some(x) = self.queue2.pop_front() {
                    self.queue1.push_back(x);
                }
            }
            self.queue2.pop_front().unwrap_or(0)
        } else {
            while self.queue1.len() > 1 {
                if let Some(x) = self.queue1.pop_front() {
                    self.queue2.push_back(x);
                }
            }
            self.queue1.pop_front().unwrap_or(0)
        }
    }
    
    fn top(&self) -> i32 {
        if self.queue1.is_empty() {
            *self.queue2.back().unwrap_or(&0)
        } else {
            *self.queue1.back().unwrap_or(&0)
        }
    }
    
    fn empty(&self) -> bool {
        self.queue1.is_empty() && self.queue2.is_empty()
    }
}
