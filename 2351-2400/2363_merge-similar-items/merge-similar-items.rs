
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut items = items1.clone();
        items.extend(items2);

        items.sort_by(|a, b| a[0].cmp(&b[0]));

        for item in items {
            if ret.is_empty() || item[0] != ret.last().unwrap()[0] {
                ret.push(item);
            } else {
                ret.last_mut().unwrap()[1] += item[1];
            }
        }

        ret

    }
}
