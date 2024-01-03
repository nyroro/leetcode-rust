
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        // 统计每个数字的出现次数

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for num in deck {
            *count_map.entry(num).or_insert(0) += 1;
        }
        
        // 找到出现次数的最大公约数

        let mut gcd = 0;
        for &count in count_map.values() {
            gcd = Self::get_gcd(gcd, count);
        }
        
        // 判断最大公约数是否大于等于2

        gcd >= 2

    }
    
    // 辗转相除法求最大公约数

    fn get_gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a

        } else {
            Self::get_gcd(b, a % b)
        }
    }
}
