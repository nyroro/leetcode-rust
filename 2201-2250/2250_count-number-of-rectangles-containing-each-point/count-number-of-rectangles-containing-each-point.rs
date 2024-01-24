
use std::collections::HashMap;

impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut table: HashMap<i32, Vec<i32>> = HashMap::new();
        for rect in rectangles {
            let x = rect[0];
            let y = rect[1];
            table.entry(y).or_insert(Vec::new()).push(x);
        }

        for (_, v) in table.iter_mut() {
            v.sort();
        }

        let mut keys: Vec<i32> = table.keys().cloned().collect();
        keys.sort_by(|a, b| b.cmp(a));

        let mut ret: Vec<i32> = Vec::new();
        for point in points {
            let mut now = 0;
            let x = point[0];
            let y = point[1];
            for &k in &keys {
                if y > k {
                    break;
                }
                if let Some(v) = table.get(&k) {
                    now += (v.len() - v.binary_search(&x).unwrap_or_else(|e| e)) as i32;
                }
            }
            ret.push(now);
        }
        ret

    }
}
