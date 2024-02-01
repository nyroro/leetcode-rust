
use std::collections::BinaryHeap;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || num <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(-num);
        }
        
        if self.max_heap.len() > self.min_heap.len() + 1 {
            self.min_heap.push(-self.max_heap.pop().unwrap());
        } else if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(-self.min_heap.pop().unwrap());
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            (*self.max_heap.peek().unwrap() as f64 - *self.min_heap.peek().unwrap() as f64) / 2.0

        } else {
            *self.max_heap.peek().unwrap() as f64

        }
    }
}
