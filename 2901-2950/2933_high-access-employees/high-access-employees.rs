
use std::collections::HashMap;

impl Solution {
    pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
        let mut table: HashMap<String, Vec<i32>> = HashMap::new();

        for access in access_times {
            let v: i32 = access[1][..2].parse().unwrap() * 60 + access[1][2..].parse().unwrap();
            table.entry(access[0].clone()).or_insert(Vec::new()).push(v);
        }

        let mut ret: Vec<String> = Vec::new();

        for (k, arr) in table.iter() {
            let mut arr = arr.clone();
            arr.sort();

            let mut x = 0;
            for i in 0..arr.len() {
                if arr[i] - arr[x] >= 60 {
                    while i > x && arr[i] - arr[x] >= 60 {
                        x += 1;
                    }
                }
                if i - x + 1 >= 3 {
                    ret.push(k.clone());
                    break;
                }
            }
        }

        ret

    }
}
