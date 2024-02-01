
struct MyCalendar {
    events: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            events: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for event in &self.events {
            if start < event.1 && end > event.0 {
                return false;
            }
        }
        self.events.push((start, end));
        true

    }
}

fn main() {
    let mut obj = MyCalendar::new();
    let ret_1: bool = obj.book(10, 20);
    let ret_2: bool = obj.book(15, 25);
    let ret_3: bool = obj.book(20, 30);
    println!("{}, {}, {}", ret_1, ret_2, ret_3);
}
