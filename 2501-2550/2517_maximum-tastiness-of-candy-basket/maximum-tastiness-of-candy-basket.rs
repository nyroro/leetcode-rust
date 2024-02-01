
impl Solution {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        let mut price = price;
        price.sort(); // 对价格数组进行排序


        let mut left = 0;
        let mut right = price.last().unwrap() - price.first().unwrap(); // 初始化左右指针


        while left <= right {
            let mid = (right - left) / 2 + left; // 使用二分查找找到最大的差值


            if Self::check(&price, mid, k) { // 检查是否存在差值大于等于mid的子数组

                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right // 返回最大的差值

    }

    fn check(price: &Vec<i32>, mid: i32, k: i32) -> bool {
        let mut count = 0;
        let mut left = 0;
        let mut right = 0;

        while right < price.len() {
            if price[right] - price[left] >= mid {
                count += 1;
                left = right;
            }
            right += 1;

            if count >= k - 1 {
                return true;
            }
        }

        false

    }
}
