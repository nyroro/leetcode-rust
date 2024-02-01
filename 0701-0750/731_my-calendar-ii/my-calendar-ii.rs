
struct MyCalendarTwo {
    events: Vec<(i32, i32)>,
    overlaps: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            events: Vec::new(),
            overlaps: Vec::new(),
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.overlaps {
            if start < e && end > s {
                return false;
            }
        }
        
        for &(s, e) in &self.events {
            if start < e && end > s {
                self.overlaps.push((start.max(s), end.min(e)));
            }
        }
        
        self.events.push((start, end));
        true

    }
}

fn main() {
    let mut obj = MyCalendarTwo::new();
    println!("{}", obj.book(10, 20)); // return True

    println!("{}", obj.book(50, 60)); // return True

    println!("{}", obj.book(10, 40)); // return True

    println!("{}", obj.book(5, 15)); // return False

    println!("{}", obj.book(5, 10)); // return True

    println!("{}", obj.book(25, 55)); // return True

}
