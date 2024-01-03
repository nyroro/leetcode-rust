


impl Solution {
    pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {
        // Sort tasks based on the end time

        let mut sorted_tasks = tasks.clone();
        sorted_tasks.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut vis = vec![0; 2010];
        let mut ans = 0;

        for task in sorted_tasks {
            let start = task[0] as usize;
            let end = task[1] as usize;
            let mut duration = task[2];

            for i in start..=end {
                duration -= vis[i];
            }

            let mut i = end as i32;
            while i >= start as i32 && duration > 0 {
                if vis[i as usize] == 0 {
                    duration -= 1;
                    ans += 1;
                    vis[i as usize] = 1;
                }
                i -= 1;
            }
        }

        ans

    }
}
