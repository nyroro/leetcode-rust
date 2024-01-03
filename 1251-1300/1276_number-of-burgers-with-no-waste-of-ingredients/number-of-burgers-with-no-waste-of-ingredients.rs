
impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        // 创建一个空的vector，用于存储巨无霸汉堡和小汉堡的数量

        let mut result = Vec::new();
        
        // 计算x和y的值

        let x = (tomato_slices - 2 * cheese_slices) / 2;
        let y = cheese_slices - x;
        
        // 将x和y的值依次添加到vector中

        if tomato_slices >= 0 && cheese_slices >= 0 && (tomato_slices - 2 * cheese_slices) % 2 == 0 && x >= 0 && y >= 0 {
            result.push(x);
            result.push(y);
        }
        
        // 返回结果

        result

    }
}
