
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut count = 0;

        for i in 0..n {
            if !visited[i] {
                Self::dfs(&is_connected, &mut visited, i);
                count += 1;
            }
        }

        count

    }

    fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, city: usize) {
        visited[city] = true;

        for neighbor in 0..is_connected.len() {
            if is_connected[city][neighbor] == 1 && !visited[neighbor] {
                Self::dfs(is_connected, visited, neighbor);
            }
        }
    }
}
