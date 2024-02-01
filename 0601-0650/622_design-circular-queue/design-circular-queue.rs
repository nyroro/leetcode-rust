
struct MyCircularQueue {
    buffer: Vec<i32>,
    front: usize,
    rear: usize,
    size: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            buffer: vec![0; k as usize],
            front: 0,
            rear: 0,
            size: 0,
        }
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false

        } else {
            self.buffer[self.rear] = value;
            self.rear = (self.rear + 1) % self.buffer.len();
            self.size += 1;
            true

        }
    }
    
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false

        } else {
            self.front = (self.front + 1) % self.buffer.len();
            self.size -= 1;
            true

        }
    }
    
    fn front(&self) -> i32 {
        if self.is_empty() {
            -1

        } else {
            self.buffer[self.front]
        }
    }
    
    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1

        } else {
            self.buffer[(self.rear + self.buffer.len() - 1) % self.buffer.len()]
        }
    }
    
    fn is_empty(&self) -> bool {
        self.size == 0

    }
    
    fn is_full(&self) -> bool {
        self.size == self.buffer.len()
    }
}

fn main() {
    let mut obj = MyCircularQueue::new(3);
    let ret_1: bool = obj.en_queue(1);
    let ret_2: bool = obj.en_queue(2);
    let ret_3: bool = obj.en_queue(3);
    let ret_4: bool = obj.en_queue(4);
    let ret_5: i32 = obj.rear();
    let ret_6: bool = obj.is_full();
    let ret_7: bool = obj.de_queue();
    let ret_8: bool = obj.en_queue(4);
    let ret_9: i32 = obj.rear();
    
    println!("{:?}", ret_1); // true

    println!("{:?}", ret_2); // true

    println!("{:?}", ret_3); // true

    println!("{:?}", ret_4); // false

    println!("{:?}", ret_5); // 3

    println!("{:?}", ret_6); // true

    println!("{:?}", ret_7); // true

    println!("{:?}", ret_8); // true

    println!("{:?}", ret_9); // 4

}
