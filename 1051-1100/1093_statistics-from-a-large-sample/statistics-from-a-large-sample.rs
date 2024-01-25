
impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let s: i32 = count.iter().sum();
        let mut arr: Vec<i32> = Vec::new();
        for (i, &t) in count.iter().enumerate() {
            if t > 0 {
                arr.push(i as i32);
            }
        }
        let minx: f64 = arr[0] as f64;
        let maxx: f64 = arr[arr.len() - 1] as f64;
        let mean: f64 = count.iter().enumerate().map(|(i, &t)| i as f64 * t as f64).sum::<f64>() / s as f64;

        let mut now: i32 = 0;
        let mut median: f64 = -1.0;
        for (i, &t) in count.iter().enumerate() {
            now += t;
            if now > s / 2 {
                median = i as f64;
                break;
            } else if now == s / 2 {
                if s % 2 == 0 {
                    let mut j = i + 1;
                    while count[j] == 0 {
                        j += 1;
                    }
                    median = (i as f64 + j as f64) / 2.0;
                    break;
                }
            }
        }

        let mut mode: i32 = -1;
        let mut modev: i32 = -1;
        for (i, &t) in count.iter().enumerate() {
            if t > modev {
                modev = t;
                mode = i as i32;
            }
        }

        vec![minx, maxx, mean, median, mode as f64]
    }
}
