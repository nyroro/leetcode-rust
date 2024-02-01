


const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 生成满足条件的数组powers

        let mut powers = vec![];
        let mut num = n;
        while num > 0 {
            let power = (num & -num) as i32;  // 获取最低位的1

            powers.push(power);
            num -= power;
        }

        // 计算每个查询的结果

        let mut result = vec![];
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            let mut product = 1;
            for i in left..=right {
                product = (product as i64 * powers[i] as i64 % MOD) as i32;
            }
            result.push(product);
        }

        result

    }
}

fn main() {
    // 测试样例

    let n = 15;
    let queries = vec![vec![0, 1], vec![2, 2], vec![0, 3]];
    let result = Solution::product_queries(n, queries);
    println!("{:?}", result);  // 输出: [2, 4, 64]
}
