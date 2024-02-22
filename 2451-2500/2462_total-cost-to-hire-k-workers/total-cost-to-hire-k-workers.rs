
use std::collections::BTreeSet;



impl Solution {
    pub fn total_cost(costs: Vec<i32>, mut k: i32, candidates: i32) -> i64 {
        let mut left: BTreeSet<(i32, usize)> = BTreeSet::new();
        let mut right: BTreeSet<(i32, usize)> = BTreeSet::new();
        
        let (mut l, mut r) = (0, costs.len() - 1);
        
        for i in 0..candidates {
            left.insert((costs[l], l));
            l += 1;
        }
        l -= 1;
        
        let mut ok = false;
        for _ in 0..candidates {
            if l < r {
                right.insert((costs[r], r));
                r -= 1;
                ok = true;
            }
        }
        if ok {
            r += 1;
        }
        
        let mut sum = 0;
        
        while k > 0 {
            if left.is_empty() {
                let (cost, index) = right.iter().next().unwrap().clone();
                sum += cost as i64;
                k -= 1;
                right.remove(&(cost, index));
            } else if right.is_empty() {
                let (cost, index) = left.iter().next().unwrap().clone();
                sum += cost as i64;
                k -= 1;
                left.remove(&(cost, index));
            } else {
                let (cost_l, index_l) = left.iter().next().unwrap().clone();
                let (cost_r, index_r) = right.iter().next().unwrap().clone();
                if cost_l <= cost_r {
                    sum += cost_l as i64;
                    k -= 1;
                    l += 1;
                    left.remove(&(cost_l, index_l));
                    if l < r {
                        left.insert((costs[l], l));
                    }
                } else {
                    sum += cost_r as i64;
                    k -= 1;
                    r -= 1;
                    right.remove(&(cost_r, index_r));
                    if l < r {
                        right.insert((costs[r], r));
                    }
                }
            }
        }
        
        sum

    }
}
