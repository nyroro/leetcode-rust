
use std::collections::HashMap;



impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut server_requests: HashMap<i32, i32> = (1..=n).map(|i| (i, 0)).collect();
        let mut sorted_logs = logs;
        sorted_logs.sort_by_key(|log| log[1]);
        let mut sorted_queries: Vec<(i32, usize)> = queries.iter().cloned().enumerate().map(|(i, query)| (query, i)).collect();
        sorted_queries.sort_by_key(|&(query, _)| query);

        let mut right = 0;
        let mut left = -1;
        let mut count_null = n;
        let mut result = vec![0; queries.len()];

        for (query, i) in sorted_queries {
            while right < sorted_logs.len() && sorted_logs[right][1] <= query {
                let server_id = sorted_logs[right][0];
                if server_requests[&server_id] == 0 {
                    count_null -= 1;
                }
                *server_requests.entry(server_id).or_insert(0) += 1;
                right += 1;
            }
            while (left + 1) < sorted_logs.len() as i32 && sorted_logs[(left + 1) as usize][1] < query - x {
                left += 1;
                let server_id = sorted_logs[left as usize][0];
                *server_requests.get_mut(&server_id).unwrap() -= 1;
                if server_requests[&server_id] == 0 {
                    count_null += 1;
                }
            }
            result[i] = count_null;
        }

        result

    }
}
