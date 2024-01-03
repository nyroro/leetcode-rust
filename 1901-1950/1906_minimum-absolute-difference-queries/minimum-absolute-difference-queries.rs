
use std::collections::HashMap;
use std::cmp::{min, max};

impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mp: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            mp.entry(*num).or_insert(Vec::new()).push(i);
        }
        
        let mut res: Vec<i32> = Vec::new();
        for query in queries {
            let (l, r) = (query[0] as usize, query[1] as usize);
            let mut pre = i32::MIN;
            let mut mi = i32::MAX;
            
            for k in (1..=100).filter(|k| mp.contains_key(k)) {
                let positions = mp.get(&k).unwrap();
                let i = positions.binary_search(&l).unwrap_or_else(|x| x);
                
                if i < positions.len() && positions[i] <= r {
                    if pre != i32::MIN {
                        mi = min(mi, k - pre);
                    }
                    pre = k;
                }
            }
            
            if mi == i32::MAX {
                res.push(-1);
            } else {
                res.push(mi);
            }
        }
        
        res

    }
}
