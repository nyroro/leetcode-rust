
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let mut answer = vec![0; n];
        let mut count = 0;

        // 从左到右计算将球从左边的盒子移动到当前盒子所需的操作次数

        for i in 0..n {
            answer[i] += count;
            if boxes.chars().nth(i).unwrap() == '1' {
                count += 1;
            }
        }

        count = 0;

        // 从右到左计算将球从右边的盒子移动到当前盒子所需的操作次数

        for i in (0..n).rev() {
            answer[i] += count;
            if boxes.chars().nth(i).unwrap() == '1' {
                count += 1;
            }
        }

        answer

    }
}
