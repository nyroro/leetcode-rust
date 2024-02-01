
struct RangeFreqQuery {
    arr: Vec<i32>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        Self { arr }
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let subarray = &self.arr[left as usize..=right as usize];
        let frequency = subarray.iter().filter(|&x| *x == value).count() as i32;
        frequency

    }
}
