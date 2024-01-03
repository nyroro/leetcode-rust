
use std::collections::HashMap;

impl Solution {
    fn find(x: i32, parent: &mut Vec<i32>) -> i32 {
        if parent[x as usize] != x {
            parent[x as usize] = Solution::find(parent[x as usize], parent);
        }
        parent[x as usize]
    }

    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let n = source.len();
        let mut parent = (0..n as i32).collect::<Vec<_>>();
        let mut source_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut target_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut result = 0;

        for swap in allowed_swaps {
            let root1 = Solution::find(swap[0], &mut parent);
            let root2 = Solution::find(swap[1], &mut parent);
            if root1 != root2 {
                parent[root1 as usize] = root2;
            }
        }

        for i in 0..n {
            let root = Solution::find(i as i32, &mut parent);
            source_map.entry(root).or_insert(Vec::new()).push(source[i]);
            target_map.entry(root).or_insert(Vec::new()).push(target[i]);
        }

        for (root, source_values) in source_map.iter() {
            let target_values = target_map.get_mut(root).unwrap();
            let mut source_count: HashMap<i32, i32> = HashMap::new();
            for &val in source_values {
                *source_count.entry(val).or_insert(0) += 1;
            }
            for &val in target_values.iter() {
                if let Some(count) = source_count.get_mut(&val) {
                    if *count > 0 {
                        *count -= 1;
                    } else {
                        result += 1;
                    }
                } else {
                    result += 1;
                }
            }
        }

        result

    }
}
