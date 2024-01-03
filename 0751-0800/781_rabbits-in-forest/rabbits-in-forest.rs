
use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut min_rabbits = 0;

        for answer in answers {
            *count_map.entry(answer).or_insert(0) += 1;
        }

        for (answer, count) in count_map {
            let group_size = answer + 1;
            let group_count = (count as f32 / group_size as f32).ceil() as i32;
            min_rabbits += group_size * group_count;
        }

        min_rabbits

    }
}
