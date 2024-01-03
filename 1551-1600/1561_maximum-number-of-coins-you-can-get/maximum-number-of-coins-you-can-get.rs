
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        piles.sort(); // 对数组进行排序

        let mut result = 0;
        let mut i = piles.len() - 2; // 从倒数第二个元素开始

        let mut count = 0;
        while count < piles.len() / 3 {
            result += piles[i];
            i -= 2;
            count += 1;
        }
        result

    }
}
