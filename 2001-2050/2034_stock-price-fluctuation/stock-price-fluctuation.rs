
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
    
    fn maximum(&self) -> i32 {
        let mut max_price = self.max_heap.peek().unwrap();
        while self.table[&max_price.1] != max_price.0 {
            max_price = self.max_heap.pop().unwrap();
            max_price = self.max_heap.peek().unwrap();
        }
        max_price.0

    }
    
    fn minimum(&self) -> i32 {
        let mut min_price = self.min_heap.peek().unwrap();
        while self.table[&min_price.1] != -min_price.0 {
            min_price = self.min_heap.pop().unwrap();
            min_price = self.min_heap.peek().unwrap();
        }
        -min_price.0

    }
}
