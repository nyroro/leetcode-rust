
impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let mut count = 0;
        let mut max_defense = 0;

        for i in (0..properties.len()).rev() {
            if properties[i][1] < max_defense {
                count += 1;
            } else {
                max_defense = properties[i][1];
            }
        }

        count

    }
}
