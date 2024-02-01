
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort(); // 对人的体重进行排序

        let mut count = 0;
        let mut i = 0;
        let mut j = people.len() - 1;

        while i <= j {
            if people[i] + people[j] <= limit {
                i += 1;
            }
            j -= 1;
            count += 1;
        }

        count

    }
}
