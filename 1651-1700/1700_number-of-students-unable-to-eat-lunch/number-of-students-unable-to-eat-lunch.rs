


impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count = vec![0, 0]; // 用于统计学生和三明治的数量

        for &student in &students {
            count[student as usize] += 1; // 统计学生的数量

        }
        
        for &sandwich in &sandwiches {
            if count[sandwich as usize] == 0 {
                break; // 如果没有学生喜欢当前类型的三明治，结束循环

            }
            count[sandwich as usize] -= 1; // 学生选择了当前类型的三明治，数量减一

        }
        
        count.iter().sum() as i32 // 返回无法吃到三明治的学生数量

    }
}

fn main() {
    let students = vec![1, 1, 0, 0];
    let sandwiches = vec![0, 1, 0, 1];
    println!("{}", Solution::count_students(students, sandwiches)); // 输出: 0


    let students2 = vec![1, 1, 1, 0, 0, 1];
    let sandwiches2 = vec![1, 0, 0, 0, 1, 1];
    println!("{}", Solution::count_students(students2, sandwiches2)); // 输出: 3

}
