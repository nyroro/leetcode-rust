
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut indegrees = vec![0; num_courses];
        let mut adjacency_list = vec![Vec::new(); num_courses];
        let mut result = Vec::new();
        let mut queue = Vec::new();
        
        // 构建入度表和邻接表

        for prerequisite in prerequisites {
            let course = prerequisite[0] as usize;
            let prerequisite_course = prerequisite[1] as usize;
            indegrees[course] += 1;
            adjacency_list[prerequisite_course].push(course);
        }
        
        // 将入度为0的节点加入队列

        for (course, &indegree) in indegrees.iter().enumerate() {
            if indegree == 0 {
                queue.push(course);
            }
        }
        
        // 拓扑排序

        while let Some(course) = queue.pop() {
            result.push(course as i32);
            for &next_course in &adjacency_list[course] {
                indegrees[next_course] -= 1;
                if indegrees[next_course] == 0 {
                    queue.push(next_course);
                }
            }
        }
        
        // 判断是否能完成所有课程的学习

        if result.len() == num_courses {
            result

        } else {
            Vec::new()
        }
    }
}
