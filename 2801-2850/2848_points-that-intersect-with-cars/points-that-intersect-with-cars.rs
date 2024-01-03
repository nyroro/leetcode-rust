
impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut covered = vec![0; 101]; // 初始化covered数组，所有元素都为0


        for car in nums {
            for i in car[0]..=car[1] {
                covered[i as usize] = 1; // 将整数点标记为被覆盖

            }
        }

        covered.iter().sum() // 统计被覆盖的整数点的数量

    }
}
