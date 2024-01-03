
use std::collections::HashMap;

struct Student {
    id: i32,
    points: i32,
}

impl Solution {
    pub fn top_students(positive_feedback: Vec<String>, negative_feedback: Vec<String>, report: Vec<String>, student_id: Vec<i32>, k: i32) -> Vec<i32> {
        let mut feedback_map: HashMap<String, i32> = HashMap::new();
        for word in positive_feedback {
            feedback_map.insert(word, 3);
        }
        for word in negative_feedback {
            feedback_map.insert(word, -1);
        }

        let mut students: HashMap<i32, i32> = HashMap::new();
        for i in 0..report.len() {
            let mut points = 0;
            for word in report[i].split_whitespace() {
                if let Some(&score) = feedback_map.get(word) {
                    points += score;
                }
            }
            let student = students.entry(student_id[i]).or_insert(0);
            *student += points;
        }

        let mut sorted_students: Vec<Student> = students.iter().map(|(&id, &points)| Student { id, points }).collect();
        sorted_students.sort_by(|a, b| b.points.cmp(&a.points).then(a.id.cmp(&b.id)));

        let mut result: Vec<i32> = Vec::new();
        for i in 0..k as usize {
            result.push(sorted_students[i].id);
        }
        result

    }
}
