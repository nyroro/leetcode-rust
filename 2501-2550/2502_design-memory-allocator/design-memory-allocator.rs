
struct Allocator {
    memory: Vec<Option<i32>>,
}

impl Allocator {
    fn new(n: i32) -> Self {
        Allocator {
            memory: vec![None; n as usize],
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut count = 0;
        let mut start = -1;

        for i in 0..self.memory.len() {
            if self.memory[i].is_none() {
                count += 1;
                if count == 1 {
                    start = i as i32;
                }
                if count == size {
                    for j in start..start + size {
                        self.memory[j as usize] = Some(m_id);
                    }
                    return start;
                }
            } else {
                count = 0;
                start = -1;
            }
        }

        -1

    }

    fn free(&mut self, m_id: i32) -> i32 {
        let mut count = 0;

        for i in 0..self.memory.len() {
            if let Some(id) = self.memory[i] {
                if id == m_id {
                    self.memory[i] = None;
                    count += 1;
                }
            }
        }

        count

    }
}
