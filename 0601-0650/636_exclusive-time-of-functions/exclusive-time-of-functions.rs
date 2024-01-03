
struct Log {
    function_id: i32,
    is_start: bool,
    timestamp: i32,
}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut stack: Vec<Log> = Vec::new();
        let mut result: Vec<i32> = vec![0; n as usize];
        let mut prev_timestamp = 0;

        for log in logs {
            let parts: Vec<&str> = log.split(':').collect();
            let function_id: i32 = parts[0].parse().unwrap();
            let is_start: bool = parts[1] == "start";
            let timestamp: i32 = parts[2].parse().unwrap();

            if is_start {
                if !stack.is_empty() {
                    let prev_function_id = stack.last().unwrap().function_id;
                    result[prev_function_id as usize] += timestamp - prev_timestamp;
                }
                stack.push(Log {
                    function_id,
                    is_start,
                    timestamp,
                });
                prev_timestamp = timestamp;
            } else {
                let log = stack.pop().unwrap();
                result[log.function_id as usize] += timestamp - prev_timestamp + 1;
                prev_timestamp = timestamp + 1;
            }
        }

        result

    }
}
