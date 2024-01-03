
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        struct Request {
            from: usize,
            to: usize,
        }

        fn backtrack(requests: &[Request], buildings: &mut [i32], index: usize, count: i32, max_count: &mut i32) {
            if index == requests.len() {
                // 检查每个建筑物的净变化是否为零

                if buildings.iter().all(|&num| num == 0) {
                    *max_count = (*max_count).max(count);
                }
                return;
            }

            let request = &requests[index];
            buildings[request.from] -= 1;
            buildings[request.to] += 1;

            backtrack(requests, buildings, index + 1, count + 1, max_count);

            buildings[request.from] += 1;
            buildings[request.to] -= 1;

            backtrack(requests, buildings, index + 1, count, max_count);
        }

        let requests: Vec<Request> = requests.into_iter().map(|r| Request { from: r[0] as usize, to: r[1] as usize }).collect();
        let mut buildings = vec![0; n as usize];
        let mut max_count = 0;

        backtrack(&requests, &mut buildings, 0, 0, &mut max_count);

        max_count

    }
}
