
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        // 定义一个比较函数，用于排序

        fn compare_bits(a: &i32, b: &i32) -> std::cmp::Ordering {
            // 计算a和b的二进制表示中1的个数

            let count_a = a.count_ones();
            let count_b = b.count_ones();
            
            // 如果1的个数相同，则按照整数本身的大小进行排序

            if count_a == count_b {
                return a.cmp(b);
            }
            
            // 否则，按照1的个数进行排序

            count_a.cmp(&count_b)
        }
        
        // 使用自定义的比较函数进行排序

        let mut sorted_arr = arr;
        sorted_arr.sort_by(compare_bits);
        
        sorted_arr

    }
}
