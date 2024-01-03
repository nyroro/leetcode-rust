
struct MyHashMap {
    map: Vec<(i32, i32)>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            map: Vec::new(),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let mut found = false;
        for pair in &mut self.map {
            if pair.0 == key {
                pair.1 = value;
                found = true;
                break;
            }
        }
        if !found {
            self.map.push((key, value));
        }
    }

    fn get(&self, key: i32) -> i32 {
        for pair in &self.map {
            if pair.0 == key {
                return pair.1;
            }
        }
        -1

    }

    fn remove(&mut self, key: i32) {
        self.map.retain(|&pair| pair.0 != key);
    }
}
