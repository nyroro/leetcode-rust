
use rand::Rng;

struct Solution {
    rects: Vec<Vec<i32>>,
    areas: Vec<i32>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut areas = Vec::new();
        let mut sum = 0;
        for rect in &rects {
            let area = (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            sum += area;
            areas.push(sum);
        }
        Solution { rects, areas }
    }

    fn pick(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(0, self.areas.last().unwrap_or(&0)) + 1;
        let index = self.areas.binary_search(&target).unwrap_or_else(|x| x);
        let rect = &self.rects[index];
        let x1 = rect[0];
        let y1 = rect[1];
        let x2 = rect[2];
        let y2 = rect[3];
        let x = rng.gen_range(x1, x2 + 1);
        let y = rng.gen_range(y1, y2 + 1);
        vec![x, y]
    }
}

fn main() {
    let rects = vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]];
    let obj = Solution::new(rects);
    let ret_1: Vec<i32> = obj.pick();
    println!("{:?}", ret_1);
}
