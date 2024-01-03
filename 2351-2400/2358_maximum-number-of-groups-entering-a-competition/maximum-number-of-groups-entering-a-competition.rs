
impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        let mut sorted_grades = grades.clone();
        sorted_grades.sort();

        let mut res = 1;
        let mut curr_size = 0;
        let mut size = 2;

        for i in 1..sorted_grades.len() {
            curr_size += 1;
            if curr_size == size {
                res = curr_size;
                size += 1;
                curr_size = 0;
            }
        }

        res

    }
}
