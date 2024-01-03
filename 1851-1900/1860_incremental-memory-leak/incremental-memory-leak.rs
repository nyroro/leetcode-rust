
impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut memory1 = memory1;
        let mut memory2 = memory2;
        let mut crash_time = 1;
        
        while memory1 >= crash_time || memory2 >= crash_time {
            if memory1 >= memory2 {
                memory1 -= crash_time;
            } else {
                memory2 -= crash_time;
            }
            crash_time += 1;
        }
        
        vec![crash_time, memory1, memory2]
    }
}
