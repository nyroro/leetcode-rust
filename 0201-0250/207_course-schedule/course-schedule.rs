
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut graph = vec![Vec::new(); num_courses];
        let mut visited = vec![0; num_courses];

        for prerequisite in prerequisites {
            let course = prerequisite[0] as usize;
            let prerequisite_course = prerequisite[1] as usize;
            graph[course].push(prerequisite_course);
        }

        fn dfs(course: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<i32>) -> bool {
            visited[course] = 1;

            for &next_course in &graph[course] {
                if visited[next_course] == 1 {
                    return false; // 存在环

                } else if visited[next_course] == 0 && !dfs(next_course, graph, visited) {
                    return false; // 存在环

                }
            }

            visited[course] = 2;
            true

        }

        for course in 0..num_courses {
            if visited[course] == 0 && !dfs(course, &graph, &mut visited) {
                return false; // 存在环

            }
        }

        true

    }
}
