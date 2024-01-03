
struct MyHashSet {
    data: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            data: vec![false; 1000001], // 由于 key 的范围是 0 到 10^6，我们创建一个大小为 1000001 的数组来存储数据

        }
    }

    fn add(&mut self, key: i32) {
        self.data[key as usize] = true; // 将对应位置的值设置为 true，表示存在

    }

    fn remove(&mut self, key: i32) {
        self.data[key as usize] = false; // 将对应位置的值设置为 false，表示不存在

    }

    fn contains(&self, key: i32) -> bool {
        self.data[key as usize] // 直接返回对应位置的值，表示是否存在

    }
}
