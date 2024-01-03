
impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // 构建课程图

        let mut graph = vec![vec![false; num_courses as usize]; num_courses as usize];
        for prerequisite in prerequisites {
            let course_a = prerequisite[0] as usize;
            let course_b = prerequisite[1] as usize;
            graph[course_a][course_b] = true;
        }
        
        // 对每个查询进行深度优先搜索

        let mut result = Vec::new();
        for query in queries {
            let start_course = query[0] as usize;
            let target_course = query[1] as usize;
            let mut visited = vec![false; num_courses as usize];
            if Self::dfs(start_course, target_course, &graph, &mut visited) {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        
        result

    }
    
    fn dfs(start: usize, target: usize, graph: &Vec<Vec<bool>>, visited: &mut Vec<bool>) -> bool {
        if start == target {
            return true;
        }
        
        visited[start] = true;
        for i in 0..graph.len() {
            if graph[start][i] && !visited[i] && Self::dfs(i, target, graph, visited) {
                return true;
            }
        }
        
        false

    }
}
