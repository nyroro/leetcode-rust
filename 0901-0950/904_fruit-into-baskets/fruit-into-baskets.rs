
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut basket = std::collections::HashMap::new();
        let (mut start, mut end) = (0, 0);
        let mut max_fruits = 0;

        while end < fruits.len() {
            let count = basket.entry(fruits[end]).or_insert(0);
            *count += 1;

            while basket.len() > 2 {
                let count = basket.get_mut(&fruits[start]).unwrap();
                *count -= 1;
                if *count == 0 {
                    basket.remove(&fruits[start]);
                }
                start += 1;
            }

            max_fruits = max_fruits.max(end - start + 1);
            end += 1;
        }

        max_fruits as i32

    }
}
