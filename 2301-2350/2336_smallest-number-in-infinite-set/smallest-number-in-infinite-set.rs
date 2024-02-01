
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

fn main() {
    let mut obj = SmallestInfiniteSet::new();
    let ret_1: i32 = obj.pop_smallest();
    obj.add_back(2);
    let ret_2: i32 = obj.pop_smallest();
    let ret_3: i32 = obj.pop_smallest();
    let ret_4: i32 = obj.pop_smallest();
    obj.add_back(1);
    let ret_5: i32 = obj.pop_smallest();
    let ret_6: i32 = obj.pop_smallest();
    let ret_7: i32 = obj.pop_smallest();
    let ret_8: i32 = obj.pop_smallest();
    println!("{:?}", [ret_1, ret_2, ret_3, ret_4, ret_5, ret_6, ret_7, ret_8]);
}
