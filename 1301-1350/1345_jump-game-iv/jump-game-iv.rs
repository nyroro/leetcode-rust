
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut jumps = vec![std::i32::MAX; n];
        let mut visited = vec![false; n];
        let mut map = std::collections::HashMap::new();
        
        for (i, &num) in arr.iter().enumerate() {
            if !map.contains_key(&num) {
                map.insert(num, vec![]);
            }
            map.get_mut(&num).unwrap().push(i);
        }
        
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        jumps[0] = 0;
        visited[0] = true;
        
        while let Some(curr) = queue.pop_front() {
            if curr == n - 1 {
                break;
            }
            
            if curr > 0 && !visited[curr - 1] {
                queue.push_back(curr - 1);
                jumps[curr - 1] = jumps[curr] + 1;
                visited[curr - 1] = true;
            }
            
            if curr < n - 1 && !visited[curr + 1] {
                queue.push_back(curr + 1);
                jumps[curr + 1] = jumps[curr] + 1;
                visited[curr + 1] = true;
            }
            
            if let Some(indices) = map.get(&arr[curr]) {
                for &next in indices {
                    if !visited[next] {
                        queue.push_back(next);
                        jumps[next] = jumps[curr] + 1;
                        visited[next] = true;
                    }
                }
                map.remove(&arr[curr]);
            }
        }
        
        jumps[n - 1]
    }
}
