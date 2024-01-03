
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // 创建一个新的数组来存储结果

        let mut result = Vec::new();
        
        // 遍历查询数组

        for query in queries {
            // 获取查询中的左右边界

            let left = query[0] as usize;
            let right = query[1] as usize;
            
            // 对子数组进行异或运算

            let mut xor_result = 0;
            for i in left..=right {
                xor_result ^= arr[i];
            }
            
            // 将结果存储在新数组中

            result.push(xor_result);
        }
        
        // 返回结果数组

        result

    }
}
