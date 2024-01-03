
// 定义一个结构体来表示数组中的元素及其索引

struct NumWithIndex {
    num: i32,
    index: usize,
}

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut nums_with_index: Vec<NumWithIndex> = nums.iter().enumerate().map(|(i, &num)| NumWithIndex { num, index: i }).collect();
        nums_with_index.sort_by_key(|x| x.num); // 按照元素值对结构体数组进行排序

        let mut marked = vec![false; nums.len()]; // 用于标记数组中的元素是否已经被标记

        let mut score: i64 = 0;

        for i in 0..nums_with_index.len() {
            let index = nums_with_index[i].index;
            if !marked[index] {
                score += nums_with_index[i].num as i64;
                marked[index] = true;
                if index > 0 {
                    marked[index - 1] = true;
                }
                if index < nums.len() - 1 {
                    marked[index + 1] = true;
                }
            }
        }

        score

    }
}
