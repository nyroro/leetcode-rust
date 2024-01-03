
impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut permutation: Vec<i32> = (1..=m).collect(); // 创建初始排列 P

        let mut result: Vec<i32> = Vec::new(); // 存储查询结果的数组


        for query in queries {
            let index = permutation.iter().position(|&x| x == query).unwrap(); // 找到查询元素在排列 P 中的位置

            result.push(index as i32); // 将位置添加到结果数组中

            permutation.remove(index); // 将查询元素从排列 P 中移除

            permutation.insert(0, query); // 将查询元素插入到排列 P 的开头

        }

        result // 返回结果数组

    }
}
