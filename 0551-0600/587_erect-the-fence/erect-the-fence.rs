
impl Solution {
    pub fn cross_product(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
        ((q[0] - p[0]) * (r[1] - p[1])) - ((q[1] - p[1]) * (r[0] - p[0]))
    }

    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees;
        let sz = trees.len();

        if sz <= 3 {
            return trees;
        } else {
            trees.sort();

            let mut up_hull: Vec<Vec<i32>> = Vec::new();
            let mut low_hull: Vec<Vec<i32>> = Vec::new();

            // for Upper Hull

            up_hull.push(trees[0].clone());
            up_hull.push(trees[1].clone());

            for j in 2..sz {
                let mut n = up_hull.len();

                while n >= 2 && Solution::cross_product(&up_hull[n - 2], &up_hull[n - 1], &trees[j]) > 0 {
                    up_hull.pop();
                    n = up_hull.len();
                }

                up_hull.push(trees[j].clone());
            }

            // for Lower Hull

            low_hull.push(trees[sz - 1].clone());
            low_hull.push(trees[sz - 2].clone());

            for j in (0..sz - 2).rev() {
                let mut m = low_hull.len();

                while m >= 2 && Solution::cross_product(&low_hull[m - 2], &low_hull[m - 1], &trees[j]) > 0 {
                    low_hull.pop();
                    m = low_hull.len();
                }

                low_hull.push(trees[j].clone());
            }

            // Finally Connecting our Upper Hull and Lower Hull

            up_hull.extend_from_slice(&low_hull);
            up_hull.sort();
            up_hull.dedup();

            up_hull

        }
    }
}
