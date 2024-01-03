
struct MyCircularDeque {
    capacity: usize,
    front: usize,
    rear: usize,
    count: usize,
    data: Vec<i32>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            capacity: k as usize,
            front: 0,
            rear: 0,
            count: 0,
            data: vec![0; k as usize],
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.capacity - 1) % self.capacity;
        self.data[self.front] = value;
        self.count += 1;
        true

    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
        self.count += 1;
        true

    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        self.count -= 1;
        true

    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear + self.capacity - 1) % self.capacity;
        self.count -= 1;
        true

    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.front]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[(self.rear + self.capacity - 1) % self.capacity]
    }

    fn is_empty(&self) -> bool {
        self.count == 0

    }

    fn is_full(&self) -> bool {
        self.count == self.capacity

    }
}
