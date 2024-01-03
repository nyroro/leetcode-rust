
use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        // 统计每个整数的出现次数

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for num in arr {
            *count_map.entry(num).or_insert(0) += 1;
        }
        
        // 按照出现次数进行排序

        let mut counts: Vec<i32> = count_map.values().cloned().collect();
        counts.sort();
        
        // 移除元素直到移除了恰好 k 个元素或者哈希表为空

        let mut remaining = counts.len();
        let mut removed = 0;
        for count in counts {
            if removed + count <= k {
                removed += count;
                remaining -= 1;
            } else {
                break;
            }
        }
        
        remaining as i32

    }
}
