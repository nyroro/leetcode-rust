
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut delta = vec![0; 1001];
        for trip in trips {
            let num = trip[0];
            let start = trip[1] as usize;
            let end = trip[2] as usize;
            delta[start] += num;
            delta[end] -= num;
        }
        let mut cur = 0;
        for num in delta {
            cur += num;
            if cur > capacity {
                return false;
            }
        }
        true

    }
}
