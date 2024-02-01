
struct SmallestInfiniteSet {
    set: Vec<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            set: (1..=1000).collect(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        self.set.sort_unstable();
        self.set.remove(0)
    }

    fn add_back(&mut self, num: i32) {
        if !self.set.contains(&num) {
            self.set.push(num);
        }
    }
}
