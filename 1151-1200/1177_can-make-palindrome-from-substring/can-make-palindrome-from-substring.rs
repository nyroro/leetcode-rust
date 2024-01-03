
use std::collections::HashMap;

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let s = s.as_bytes();
        let mut prefix_sum = vec![HashMap::new(); s.len() + 1];
        let mut result = Vec::new();

        // 计算前缀和

        for i in 0..s.len() {
            let mut count = prefix_sum[i].clone();
            count.entry(s[i] - b'a').and_modify(|e| *e += 1).or_insert(1);
            prefix_sum[i + 1] = count;
        }

        for query in queries {
            let (left, right, k) = (query[0] as usize, query[1] as usize, query[2] as usize);
            let mut odd_count = 0;

            // 统计子串中每个字母的出现次数

            for j in 0..26 {
                let total = *prefix_sum[right + 1].get(&j).unwrap_or(&0) - *prefix_sum[left].get(&j).unwrap_or(&0);
                if total % 2 != 0 {
                    odd_count += 1;
                }
            }

            // 判断最多需要替换的字符数是否小于等于k

            if (right - left + 1) % 2 == 0 {
                result.push(odd_count / 2 <= k);
            } else {
                result.push((odd_count - 1) / 2 <= k);
            }
        }

        result

    }
}
