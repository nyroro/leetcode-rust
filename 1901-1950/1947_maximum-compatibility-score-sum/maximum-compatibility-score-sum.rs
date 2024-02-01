
impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let mut assigned = vec![false; mentors.len()];
        let mut max_sum = 0;
        
        fn calculate_compatibility(student: &Vec<i32>, mentor: &Vec<i32>) -> i32 {
            student.iter().zip(mentor.iter()).filter(|(&a, &b)| a == b).count() as i32

        }
        
        fn backtrack(students: &Vec<Vec<i32>>, mentors: &Vec<Vec<i32>>, assigned: &mut Vec<bool>, student_idx: usize, sum: i32, max_sum: &mut i32) {
            if student_idx == students.len() {
                *max_sum = (*max_sum).max(sum);
                return;
            }
            
            for (i, mentor) in mentors.iter().enumerate() {
                if !assigned[i] {
                    assigned[i] = true;
                    let compatibility = calculate_compatibility(&students[student_idx], mentor);
                    backtrack(students, mentors, assigned, student_idx + 1, sum + compatibility, max_sum);
                    assigned[i] = false;
                }
            }
        }
        
        backtrack(&students, &mentors, &mut assigned, 0, 0, &mut max_sum);
        
        max_sum

    }
}
