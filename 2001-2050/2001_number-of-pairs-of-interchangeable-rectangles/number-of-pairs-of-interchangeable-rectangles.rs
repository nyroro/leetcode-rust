
use std::collections::HashMap;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut ratio_count: HashMap<(i32, i32), i64> = HashMap::new();
        let mut result: i64 = 0;

        for rectangle in rectangles {
            let width = rectangle[0];
            let height = rectangle[1];
            let gcd = Solution::gcd(width, height);
            let ratio = (width / gcd, height / gcd);
            *ratio_count.entry(ratio).or_insert(0) += 1;
        }

        for (_, count) in ratio_count {
            if count > 1 {
                result += count * (count - 1) / 2;
            }
        }

        result

    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a

        } else {
            Solution::gcd(b, a % b)
        }
    }
}
