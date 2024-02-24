
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Clone)]
struct Item {
    profit: i64,
    category: i64,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.profit.cmp(&self.profit)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_maximum_elegance(items: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut mp: HashMap<i64, i64> = HashMap::new();
        let mut pq: BinaryHeap<Item> = BinaryHeap::new();
        let mut items: Vec<Item> = items.into_iter().map(|v| Item { profit: v[0] as i64, category: v[1] as i64 }).collect();
        items.sort_by(|a, b| b.profit.cmp(&a.profit));
        let mut ele: i64 = 0;
        let mut ans: i64 = 0;
        
        for i in 0..k as usize {
            ele = ele.checked_add(items[i].profit).unwrap_or(i64::MAX);
            *mp.entry(items[i].category).or_insert(0) += 1;
            pq.push(items[i].clone()); // Use clone to push a copy into the priority queue

        }
        
        ans = ele.checked_add((mp.len() as i64).pow(2)).unwrap_or(i64::MAX);
        let n = items.len();
        
        for i in k as usize..n {
            if let Some(item) = pq.peek() {
                if mp.contains_key(&items[i].category) {
                    continue;
                }
                let mut it = pq.pop().unwrap();
                while !pq.is_empty() && mp[&it.category] == 1 {
                    it = pq.pop().unwrap();
                }
                if pq.is_empty() {
                    break;
                }
                ele = ele.checked_add(items[i].profit).unwrap_or(i64::MAX);
                ele = ele.checked_sub(it.profit).unwrap_or(i64::MIN);
                mp.insert(it.category, mp[&it.category] - 1);
                mp.insert(items[i].category, mp.get(&items[i].category).unwrap_or(&0) + 1);
                let freq = (mp.len() as i64).checked_mul(mp.len() as i64).unwrap_or(i64::MAX);
                ans = ans.max(ele.checked_add(freq).unwrap_or(i64::MAX));
            }
        }
        
        ans

    }
}
