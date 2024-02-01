
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

struct SeatManager {
    available_seats: BinaryHeap<Reverse<i32>>,
    reserved_seats: HashSet<i32>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        let mut available_seats = BinaryHeap::new();
        for i in 1..=n {
            available_seats.push(Reverse(i));
        }
        SeatManager {
            available_seats,
            reserved_seats: HashSet::new(),
        }
    }
    
    fn reserve(&mut self) -> i32 {
        let seat = self.available_seats.pop().unwrap().0;
        self.reserved_seats.insert(seat);
        seat

    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.reserved_seats.remove(&seat_number);
        self.available_seats.push(Reverse(seat_number));
    }
}

fn main() {
    let mut obj = SeatManager::new(5);
    let ret_1: i32 = obj.reserve();
    println!("{}", ret_1); // Output: 1

    let ret_2: i32 = obj.reserve();
    println!("{}", ret_2); // Output: 2

    obj.unreserve(2);
    let ret_3: i32 = obj.reserve();
    println!("{}", ret_3); // Output: 2

    let ret_4: i32 = obj.reserve();
    println!("{}", ret_4); // Output: 3

    let ret_5: i32 = obj.reserve();
    println!("{}", ret_5); // Output: 4

    let ret_6: i32 = obj.reserve();
    println!("{}", ret_6); // Output: 5

    obj.unreserve(5);
}
