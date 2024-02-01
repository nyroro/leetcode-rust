
struct MapSum {
    map: std::collections::HashMap<String, i32>,
}

impl MapSum {
    // 创建一个新的MapSum对象

    fn new() -> Self {
        MapSum {
            map: std::collections::HashMap::new(),
        }
    }
    
    // 向map中插入键值对，如果键已存在，则更新对应的值

    fn insert(&mut self, key: String, val: i32) {
        self.map.insert(key, val);
    }
    
    // 返回所有以prefix为前缀的键对应的值的总和

    fn sum(&self, prefix: String) -> i32 {
        let mut sum = 0;
        for (key, val) in &self.map {
            if key.starts_with(&prefix) {
                sum += val;
            }
        }
        sum

    }
}

fn main() {
    let mut obj = MapSum::new();
    obj.insert("apple".to_string(), 3);
    let ret_1 = obj.sum("ap".to_string());
    println!("{}", ret_1); // 输出3

    obj.insert("app".to_string(), 2);
    let ret_2 = obj.sum("ap".to_string());
    println!("{}", ret_2); // 输出5

}
