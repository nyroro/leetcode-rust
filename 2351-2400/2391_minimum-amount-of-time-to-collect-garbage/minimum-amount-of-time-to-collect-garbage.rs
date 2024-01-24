
struct GarbageTruck {
    garbage_type: char,
    position: usize,
}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut trucks = vec![
            GarbageTruck { garbage_type: 'M', position: 0 },
            GarbageTruck { garbage_type: 'P', position: 0 },
            GarbageTruck { garbage_type: 'G', position: 0 },
        ];
        
        let mut total_time = 0;
        
        for (i, g) in garbage.iter().enumerate() {
            total_time += g.len() as i32;
            for (j, c) in g.chars().enumerate() {
                match c {
                    'M' => trucks[0].position = i,
                    'P' => trucks[1].position = i,
                    'G' => trucks[2].position = i,
                    _ => (),
                }
            }
        }
        
        for truck in trucks.iter() {
            total_time += travel.iter().take(truck.position).sum::<i32>();
        }
        
        total_time

    }
}
