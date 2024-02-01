
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let size = 1 << n; // 计算排列的长度

        let mut result = Vec::with_capacity(size); // 创建一个容量为size的向量


        // 生成排列

        for i in 0..size {
            result.push(i ^ (i >> 1)); // 使用异或运算生成满足条件的排列

        }

        // 找到start的位置并移动到开头

        let start_index = result.iter().position(|&x| x == start).unwrap();
        result.rotate_left(start_index);

        result // 返回结果

    }
}
