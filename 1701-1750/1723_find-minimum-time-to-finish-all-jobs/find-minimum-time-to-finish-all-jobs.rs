
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let mut workers = vec![0; k as usize];
        let mut result = i32::MAX;
        Self::backtrack(0, &jobs, &mut workers, &mut result);
        result

    }
    
    fn backtrack(idx: usize, jobs: &Vec<i32>, workers: &mut Vec<i32>, result: &mut i32) {
        if idx == jobs.len() {
            *result = (*result).min(*workers.iter().max().unwrap());
            return;
        }
        
        for i in 0..workers.len() {
            workers[i] += jobs[idx];
            Self::backtrack(idx + 1, jobs, workers, result);
            workers[i] -= jobs[idx];
            if workers[i] == 0 {
                break;
            }
        }
    }
}
