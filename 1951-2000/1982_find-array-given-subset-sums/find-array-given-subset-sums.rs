
use std::collections::HashMap;



impl Solution {
    pub fn recover_array(n: i32, mut sums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        sums.sort();

        while sums.len() > 1 {
            let num = sums.last().unwrap() - sums[sums.len() - 2];
            let mut count_map: HashMap<i32, i32> = HashMap::new();
            for &x in &sums {
                *count_map.entry(x).or_insert(0) += 1;
            }

            let mut excluding: Vec<i32> = Vec::new();
            let mut including: Vec<i32> = Vec::new();

            for &x in &sums {
                if let Some(count) = count_map.get_mut(&x) {
                    if *count > 0 {
                        excluding.push(x);
                        including.push(x + num);
                        *count -= 1;
                        *count_map.get_mut(&(x + num)).unwrap() -= 1;
                    }
                }
            }

            if excluding.contains(&0) {
                sums = excluding;
                res.push(num);
            } else {
                sums = including;
                res.push(-1 * num);
            }
        }

        res

    }
}
