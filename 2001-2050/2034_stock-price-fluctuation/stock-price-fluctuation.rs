
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct StockPrice {
    max_heap: BinaryHeap<(i32, i32)>, // 存储（价格，时间戳）的最大堆

    min_heap: BinaryHeap<(i32, i32)>, // 存储（-价格，时间戳）的最小堆

    table: HashMap<i32, i32>, // 存储时间戳和股价的哈希表

    latest: i32, // 最新时间戳

}

impl StockPrice {
    fn new() -> Self {
        StockPrice {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
            table: HashMap::new(),
            latest: 0,
        }
    }
    
    fn update(&mut self, timestamp: i32, price: i32) {
        self.max_heap.push((price, timestamp));
        self.min_heap.push((-price, timestamp));
        self.table.insert(timestamp, price);
        self.latest = self.latest.max(timestamp);
    }
    
    fn current(&self) -> i32 {
        *self.table.get(&self.latest).unwrap()
    }
    
    fn maximum(&mut self) -> i32 {
        while let Some(max_price) = self.max_heap.peek() {
            if self.table[&max_price.1] == max_price.0 {
                return max_price.0;
            } else {
                self.max_heap.pop();
            }
        }
        0 // 返回默认值，表示没有找到最大值

    }
    
    fn minimum(&mut self) -> i32 {
        while let Some(min_price) = self.min_heap.peek() {
            if self.table[&min_price.1] == -min_price.0 {
                return -min_price.0;
            } else {
                self.min_heap.pop();
            }
        }
        0 // 返回默认值，表示没有找到最小值

    }
}
