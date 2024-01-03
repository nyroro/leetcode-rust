
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 按照身高从高到低，k从小到大排序

        let mut sorted_people = people.clone();
        sorted_people.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });
        
        let mut queue = Vec::new();
        
        // 将每个人插入到正确的位置

        for person in sorted_people {
            let index = person[1] as usize;
            queue.insert(index, person);
        }
        
        queue

    }
}
